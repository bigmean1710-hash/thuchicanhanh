#![cfg(test)]
extern crate std;

use super::*;
use soroban_sdk::{symbol_short, testutils::Address as _, Address, Env, String};

#[test]
fn test_hello() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let result = client.hello(&String::from_str(&env, "Minh"));
    assert_eq!(result.len(), 3);
}

#[test]
fn test_thu_chi_co_ban() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);
    let user = Address::generate(&env);
    let salary = symbol_short!("salary");
    let food = symbol_short!("food");

    client.initialize(&user);

    // Thu lương 25 triệu
    client.thu(&user, &salary, &25_000_000);
    assert_eq!(client.so_du(&user, &salary), 25_000_000);

    // Chi ăn uống 850 nghìn
    client.chi(&user, &food, &850_000);
    assert_eq!(client.so_du(&user, &food), 24_150_000);

    let report = client.summary(&user);
    assert_eq!(report.tong_thu, 25_000_000);
    assert_eq!(report.tong_chi, 850_000);
    assert_eq!(report.con_lai, 24_150_000);
}

#[test]
fn test_chuyen_muc_va_ngan_sach() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);
    let user = Address::generate(&env);
    let salary = symbol_short!("salary");
    let save = symbol_short!("save");
    let food = symbol_short!("food");

    client.initialize(&user);
    client.thu(&user, &salary, &10_000_000);
    client.chuyen_muc(&user, &salary, &save, &3_000_000);

    assert_eq!(client.so_du(&user, &salary), 7_000_000);
    assert_eq!(client.so_du(&user, &save), 3_000_000);

    client.dat_ngan_sach(&user, &food, &2_000_000);
    client.chi(&user, &food, &500_000);
    assert_eq!(client.ngan_sach_con_lai(&user, &food), 1_500_000);
}

#[test]
fn test_alias_deposit_withdraw() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);
    let user = Address::generate(&env);
    let rent = symbol_short!("rent");

    client.initialize(&user);
    client.deposit(&user, &rent, &5_000_000);
    client.withdraw(&user, &rent, &1_000_000);

    assert_eq!(client.balance(&user, &rent), 4_000_000);
}

#[test]
#[should_panic(expected = "insufficient balance")]
fn test_chi_vuot_so_du() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);
    let user = Address::generate(&env);

    client.initialize(&user);
    client.chi(&user, &symbol_short!("shop"), &100);
}

#[test]
#[should_panic(expected = "not initialized")]
fn test_thu_truoc_khi_init() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);
    let user = Address::generate(&env);

    client.thu(&user, &symbol_short!("salary"), &1000);
}
