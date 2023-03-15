#![no_std]

use soroban_sdk::{contractimpl, Bytes, BytesN, Env, RawVal, Symbol, Vec};

pub struct Deployer;

#[contractimpl]
impl Deployer {
    pub fn deploy(
        env: Env,
        salt: Bytes,
        wasm_hash: BytesN<32>,
        init_args: Vec<RawVal>,
    ) -> (BytesN<32>, RawVal) {
        let id = env
            .deployer()
            .with_current_contract(&salt)
            .deploy(&wasm_hash);
        let init_fn = Symbol::short("init");
        let res: RawVal = env.invoke_contract(&id, &init_fn, init_args);
        (id, res)
    }
}

mod test;