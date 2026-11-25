#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::Address as _, vec, Env};

#[test]
fn ronda_completa_y_cambia_receptor() {
    let env = Env::default();
    env.mock_all_auths();
    let id = env.register(Tanda, ());
    let c = TandaClient::new(&env, &id);

    let admin = Address::generate(&env);
    let a = Address::generate(&env);
    let b = Address::generate(&env);
    c.init(&admin, &vec![&env, a.clone(), b.clone()], &1000);

    assert_eq!(c.recipient(), a);
    c.pay(&a);
    assert!(!c.is_round_complete());
    c.pay(&b);
    assert!(c.is_round_complete());

    c.next_round();
    assert_eq!(c.recipient(), b);
}

#[test]
#[should_panic(expected = "faltan pagos")]
fn no_avanza_sin_pagos() {
    let env = Env::default();
    env.mock_all_auths();
    let id = env.register(Tanda, ());
    let c = TandaClient::new(&env, &id);
    let admin = Address::generate(&env);
    let a = Address::generate(&env);
    c.init(&admin, &vec![&env, a], &1000);
    c.next_round();
}