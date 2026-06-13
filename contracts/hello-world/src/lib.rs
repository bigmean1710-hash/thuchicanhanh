#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Env, Map, String, Symbol, Vec, vec
};

/// Khoa luu tru on-chain
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Balances(Address),
    TotalThu(Address),
    TotalChi(Address),
    ChiTheoMuc(Address),
}

/// Bao cao thu chi
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ThuChiSummary {
    pub tong_thu: i128,
    pub tong_chi: i128,
    pub con_lai: i128,
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    /// Mo so thu chi (1 lan / moi vi)
    pub fn initialize(env: Env, user: Address) {
        user.require_auth();

        let key = DataKey::Balances(user.clone());
        if env.storage().persistent().has(&key) {
            panic!("already initialized");
        }

        env.storage()
            .persistent()
            .set(&key, &Map::<Symbol, i128>::new(&env));
        env.storage()
            .persistent()
            .set(&DataKey::TotalThu(user.clone()), &0i128);
        env.storage()
            .persistent()
            .set(&DataKey::TotalChi(user.clone()), &0i128);
        env.storage()
            .persistent()
            .set(&DataKey::ChiTheoMuc(user), &Map::<Symbol, i128>::new(&env));
    }

    /// Ghi THU nhap (luong, thuong, freelance...)
    pub fn thu(env: Env, user: Address, category: Symbol, amount: i128) {
        user.require_auth();
        do_thu(&env, &user, &category, amount);
    }

    /// Ghi CHI tieu (an uong, thue nha, mua sam...)
    /// `category` = loai chi tieu; tien tru tu tong so du cac danh muc
    pub fn chi(env: Env, user: Address, category: Symbol, amount: i128) {
        user.require_auth();
        do_chi(&env, &user, &category, amount);
    }

    /// Chuyen tien giua 2 danh muc (VD: salary -> save)
    pub fn chuyen_muc(env: Env, user: Address, from: Symbol, to: Symbol, amount: i128) {
        user.require_auth();
        do_chuyen_muc(&env, &user, &from, &to, amount);
    }

    /// So du con lai cua mot danh muc
    pub fn so_du(env: Env, user: Address, category: Symbol) -> i128 {
        load_balances(&env, &user)
            .get(category)
            .unwrap_or(0)
    }

    /// Tong so du tat ca danh muc
    pub fn tong_so_du(env: Env, user: Address) -> i128 {
        calc_tong_so_du(&env, &user)
    }

    /// Bao cao tong thu / tong chi / con lai
    pub fn summary(env: Env, user: Address) -> ThuChiSummary {
        ThuChiSummary {
            tong_thu: load_total_thu(&env, &user),
            tong_chi: load_total_chi(&env, &user),
            con_lai: calc_tong_so_du(&env, &user),
        }
    }

    /// Dat han muc chi tieu thang cho danh muc
    pub fn dat_ngan_sach(env: Env, user: Address, category: Symbol, limit: i128) {
        user.require_auth();
        assert_positive(limit);
        require_initialized(&env, &user);

        let key = budget_key(&user, &category);
        env.storage().persistent().set(&key, &limit);
    }

    /// Ngan sach con lai cua danh muc
    pub fn ngan_sach_con_lai(env: Env, user: Address, category: Symbol) -> i128 {
        let key = budget_key(&user, &category);
        let limit: i128 = env.storage().persistent().get(&key).unwrap_or(0);

        if limit == 0 {
            return 0;
        }

        let da_chi = load_chi_theo_muc(&env, &user)
            .get(category)
            .unwrap_or(0);

        if limit > da_chi {
            limit - da_chi
        } else {
            0
        }
    }

    // Alias tieng Anh (tuong thich web / CLI)
    pub fn deposit(env: Env, user: Address, category: Symbol, amount: i128) {
        user.require_auth();
        do_thu(&env, &user, &category, amount);
    }

    pub fn withdraw(env: Env, user: Address, category: Symbol, amount: i128) {
        user.require_auth();
        do_chi(&env, &user, &category, amount);
    }

    pub fn balance(env: Env, user: Address, category: Symbol) -> i128 {
        load_balances(&env, &user)
            .get(category)
            .unwrap_or(0)
    }

    pub fn total_balance(env: Env, user: Address) -> i128 {
        calc_tong_so_du(&env, &user)
    }

    pub fn transfer_category(env: Env, user: Address, from: Symbol, to: Symbol, amount: i128) {
        user.require_auth();
        do_chuyen_muc(&env, &user, &from, &to, amount);
    }

    pub fn set_budget(env: Env, user: Address, category: Symbol, limit: i128) {
        user.require_auth();
        assert_positive(limit);
        require_initialized(&env, &user);

        let key = budget_key(&user, &category);
        env.storage().persistent().set(&key, &limit);
    }

    pub fn budget_remaining(env: Env, user: Address, category: Symbol) -> i128 {
        let key = budget_key(&user, &category);
        let limit: i128 = env.storage().persistent().get(&key).unwrap_or(0);

        if limit == 0 {
            return 0;
        }

        let da_chi = load_chi_theo_muc(&env, &user)
            .get(category)
            .unwrap_or(0);

        if limit > da_chi {
            limit - da_chi
        } else {
            0
        }
    }

    /// Ham mau Soroban
    pub fn hello(env: Env, to: String) -> Vec<String> {
        vec![
            &env,
            String::from_str(&env, "Hello"),
            to,
            String::from_str(&env, "Personal Finance"),
        ]
    }
}

// ─── Logic chung (goi tu nhieu ham public) ────────────────────────────────────

fn do_thu(env: &Env, user: &Address, category: &Symbol, amount: i128) {
    assert_positive(amount);
    require_initialized(env, user);

    let mut balances = load_balances(env, user);
    let current = balances.get(category.clone()).unwrap_or(0);
    balances.set(category.clone(), current + amount);
    save_balances(env, user, &balances);

    let tong_thu = load_total_thu(env, user) + amount;
    env.storage()
        .persistent()
        .set(&DataKey::TotalThu(user.clone()), &tong_thu);
}

fn do_chi(env: &Env, user: &Address, category: &Symbol, amount: i128) {
    assert_positive(amount);
    require_initialized(env, user);

    if calc_tong_so_du(env, user) < amount {
        panic!("insufficient balance");
    }

    let mut balances = load_balances(env, user);
    deduct_amount(&mut balances, amount);
    save_balances(env, user, &balances);

    let tong_chi = load_total_chi(env, user) + amount;
    env.storage()
        .persistent()
        .set(&DataKey::TotalChi(user.clone()), &tong_chi);

    let mut chi_muc = load_chi_theo_muc(env, user);
    let spent = chi_muc.get(category.clone()).unwrap_or(0);
    chi_muc.set(category.clone(), spent + amount);
    env.storage()
        .persistent()
        .set(&DataKey::ChiTheoMuc(user.clone()), &chi_muc);
}

fn do_chuyen_muc(env: &Env, user: &Address, from: &Symbol, to: &Symbol, amount: i128) {
    assert_positive(amount);

    if from == to {
        panic!("categories must differ");
    }

    require_initialized(env, user);

    let mut balances = load_balances(env, user);
    let from_bal = balances.get(from.clone()).unwrap_or(0);

    if from_bal < amount {
        panic!("insufficient balance");
    }

    let to_bal = balances.get(to.clone()).unwrap_or(0);
    balances.set(from.clone(), from_bal - amount);
    balances.set(to.clone(), to_bal + amount);
    save_balances(env, user, &balances);
}

/// Tru tien lan luot tu cac danh muc co so du
fn deduct_amount(balances: &mut Map<Symbol, i128>, mut amount: i128) {
    let keys: Vec<Symbol> = balances.keys();

    for key in keys.iter() {
        if amount == 0 {
            break;
        }

        let bal = balances.get(key.clone()).unwrap_or(0);
        if bal == 0 {
            continue;
        }

        if bal >= amount {
            balances.set(key.clone(), bal - amount);
            amount = 0;
        } else {
            balances.set(key.clone(), 0);
            amount -= bal;
        }
    }

    if amount > 0 {
        panic!("insufficient balance");
    }
}

fn calc_tong_so_du(env: &Env, user: &Address) -> i128 {
    let balances = load_balances(env, user);
    let mut total: i128 = 0;

    for (_, amount) in balances.iter() {
        total += amount;
    }

    total
}

fn budget_key(user: &Address, category: &Symbol) -> (Symbol, Address, Symbol) {
    (symbol_short!("budget"), user.clone(), category.clone())
}

fn assert_positive(amount: i128) {
    if amount <= 0 {
        panic!("amount must be positive");
    }
}

fn require_initialized(env: &Env, user: &Address) {
    if !env
        .storage()
        .persistent()
        .has(&DataKey::Balances(user.clone()))
    {
        panic!("not initialized — call initialize() first");
    }
}

fn load_balances(env: &Env, user: &Address) -> Map<Symbol, i128> {
    env.storage()
        .persistent()
        .get(&DataKey::Balances(user.clone()))
        .unwrap_or_else(|| Map::new(env))
}

fn save_balances(env: &Env, user: &Address, balances: &Map<Symbol, i128>) {
    env.storage()
        .persistent()
        .set(&DataKey::Balances(user.clone()), balances);
}

fn load_total_thu(env: &Env, user: &Address) -> i128 {
    env.storage()
        .persistent()
        .get(&DataKey::TotalThu(user.clone()))
        .unwrap_or(0)
}

fn load_total_chi(env: &Env, user: &Address) -> i128 {
    env.storage()
        .persistent()
        .get(&DataKey::TotalChi(user.clone()))
        .unwrap_or(0)
}

fn load_chi_theo_muc(env: &Env, user: &Address) -> Map<Symbol, i128> {
    env.storage()
        .persistent()
        .get(&DataKey::ChiTheoMuc(user.clone()))
        .unwrap_or_else(|| Map::new(env))
}

mod test;
