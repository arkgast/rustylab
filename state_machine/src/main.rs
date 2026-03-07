mod balances;
mod errors;
mod proof_of_existene;
mod support;
mod system;
mod types;

use crate::support::Dispatch;

#[derive(Debug)]
#[macros::runtime]
pub struct Runtime {
    system: system::Pallet<Self>,
    balances: balances::Pallet<Self>,
    proof_of_existene: proof_of_existene::Pallet<Self>,
}

impl system::Config for Runtime {
    type AccountId = types::AccountId;
    type BlockNumber = types::BlockNumber;
    type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
    type Balance = types::Balance;
}

impl proof_of_existene::Config for Runtime {
    type Content = types::Content;
}

fn main() {
    let alice = "alice".to_string();
    let bob = "bob".to_string();
    let content = "content".to_string();

    let mut runtime = Runtime::new();
    runtime.balances.set_balance(&alice, 100_000);

    let block = types::Block {
        header: support::Header { block_number: 1 },
        extrinsics: vec![
            types::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::balances(balances::Call::transfer {
                    to: bob.clone(),
                    amount: 10,
                }),
            },
            types::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::balances(balances::Call::transfer {
                    to: bob.clone(),
                    amount: 100,
                }),
            },
            types::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::proof_of_existene(proof_of_existene::Call::create_claim {
                    claim: content.clone(),
                }),
            },
            types::Extrinsic {
                caller: bob.clone(),
                call: RuntimeCall::proof_of_existene(proof_of_existene::Call::create_claim {
                    claim: content.clone(),
                }),
            },
        ],
    };
    runtime.execute_block(block).expect("Invalid block");

    println!("\nRuntime state: {:#?}", runtime);
}
