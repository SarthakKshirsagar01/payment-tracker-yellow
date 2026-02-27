#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Address, Env, Symbol, Vec,
};

#[contracttype]
#[derive(Clone)]
pub struct Payment {
    pub from: Address,
    pub amount: i128,
}

#[contract]
pub struct PaymentTracker;

#[contractimpl]
impl PaymentTracker {
    pub fn record_payment(env: Env, from: Address, amount: i128) {
        // Require auth (important for Level 2 quality)
        from.require_auth();

        let key: Symbol = symbol_short!("PAYMENTS");

        let mut payments: Vec<Payment> =
            env.storage().instance().get(&key).unwrap_or(Vec::new(&env));

        // ✅ clone to avoid move error
        payments.push_back(Payment {
            from: from.clone(),
            amount,
        });

        env.storage().instance().set(&key, &payments);

        // Emit event
        env.events().publish(
            (symbol_short!("payment"),),
            (from, amount),
        );
    }

    pub fn get_payments(env: Env) -> Vec<Payment> {
        env.storage()
            .instance()
            .get(&symbol_short!("PAYMENTS"))
            .unwrap_or(Vec::new(&env))
    }
}