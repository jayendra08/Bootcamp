#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol, Address, String};

#[contract]
pub struct TokenCreator;

#[contractimpl]
impl TokenCreator {

    // Initialize token metadata
    pub fn init(
        env: Env,
        admin: Address,
        name: String,
        symbol: String,
        decimals: u32,
    ) {
        admin.require_auth();

        env.storage().instance().set(&symbol_short!("ADMIN"), &admin);
        env.storage().instance().set(&symbol_short!("NAME"), &name);
        env.storage().instance().set(&symbol_short!("SYMBOL"), &symbol);
        env.storage().instance().set(&symbol_short!("DEC"), &decimals);
    }

    // Mint tokens to a user
    pub fn mint(env: Env, to: Address, amount: i128) {
        let admin: Address = env.storage().instance().get(&symbol_short!("ADMIN")).unwrap();
        admin.require_auth();

        let key = (symbol_short!("BAL"), to.clone());
        let balance: i128 = env.storage().instance().get(&key).unwrap_or(0);

        env.storage().instance().set(&key, &(balance + amount));
    }

    // Transfer tokens
    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();

        let from_key = (symbol_short!("BAL"), from.clone());
        let to_key = (symbol_short!("BAL"), to.clone());

        let from_balance: i128 = env.storage().instance().get(&from_key).unwrap_or(0);
        let to_balance: i128 = env.storage().instance().get(&to_key).unwrap_or(0);

        if from_balance < amount {
            panic!("Insufficient balance");
        }

        env.storage().instance().set(&from_key, &(from_balance - amount));
        env.storage().instance().set(&to_key, &(to_balance + amount));
    }

    // Check balance
    pub fn balance(env: Env, user: Address) -> i128 {
        let key = (symbol_short!("BAL"), user);
        env.storage().instance().get(&key).unwrap_or(0)
    }

    // Get token metadata
    pub fn get_metadata(env: Env) -> (String, String, u32) {
        let name: String = env.storage().instance().get(&symbol_short!("NAME")).unwrap();
        let symbol: String = env.storage().instance().get(&symbol_short!("SYMBOL")).unwrap();
        let decimals: u32 = env.storage().instance().get(&symbol_short!("DEC")).unwrap();

        (name, symbol, decimals)
    }
}