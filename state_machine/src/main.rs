mod balances;
mod errors;
mod proof_of_existene;
mod support;
mod system;
mod types;

use crate::support::Dispatch;

pub enum RuntimeCall {
    Balances(balances::Call<Runtime>),
    ProofOfExistence(proof_of_existene::Call<Runtime>),
}

#[derive(Debug)]
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

impl support::Dispatch for Runtime {
    type Caller = <Runtime as system::Config>::AccountId;
    type Call = RuntimeCall;
    type Error = errors::RuntimeError;

    fn dispatch(
        &mut self,
        caller: Self::Caller,
        runtime_call: Self::Call,
    ) -> support::DispatchResult<Self::Error> {
        match runtime_call {
            RuntimeCall::Balances(call) => self.balances.dispatch(caller, call)?,
            RuntimeCall::ProofOfExistence(call) => self.proof_of_existene.dispatch(caller, call)?,
        }
        Ok(())
    }
}

#[allow(clippy::new_without_default)]
impl Runtime {
    pub fn new() -> Self {
        Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
            proof_of_existene: proof_of_existene::Pallet::new(),
        }
    }

    pub fn execute_block(
        &mut self,
        block: types::Block,
    ) -> support::DispatchResult<errors::SystemError> {
        self.system.inc_block_number()?;

        // An extrinsic error is not enough to trigger the block to be invalid. We capture the
        // result, and emit an error message if one is emitted.
        for (idx, support::Extrinsic { caller, call }) in block.extrinsics.into_iter().enumerate() {
            self.system.inc_nonce(&caller)?;

            let res = self.dispatch(caller, call).map_err(|e| {
                eprintln!(
                    "Extrinsic Error\n\tBlock Number: {}\n\tExtrinsic Number: {}\n\tError: {}",
                    block.header.block_number, idx, e
                )
            });

            println!("{:?}", res);
        }

        Ok(())
    }
}

fn main() {
    let alice = "alice".to_string();
    let bob = "bob".to_string();

    let mut runtime = Runtime::new();
    runtime.balances.set_balance(&alice, 100_000);

    // Tx #1
    let block = types::Block {
        header: support::Header {
            block_number: runtime.system.block_number(),
        },
        extrinsics: vec![types::Extrinsic {
            caller: alice.clone(),
            call: RuntimeCall::Balances(balances::Call::Transfer {
                to: bob.clone(),
                amount: 10,
            }),
        }],
    };
    runtime.execute_block(block).expect("Invalid block");

    // Tx #2
    let block = types::Block {
        header: support::Header {
            block_number: runtime.system.block_number(),
        },
        extrinsics: vec![types::Extrinsic {
            caller: alice.clone(),
            call: RuntimeCall::Balances(balances::Call::Transfer {
                to: bob.clone(),
                amount: 100,
            }),
        }],
    };
    runtime.execute_block(block).expect("Invalid block");

    println!("Runtime state: {:#?}", runtime);
}
