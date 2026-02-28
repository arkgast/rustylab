mod balances;
mod support;
mod system;
mod types;

use crate::support::Dispatch;

pub enum RuntimeCall {
    BalancesTransfer {
        to: types::AccountId,
        amount: types::Balance,
    },
}

#[derive(Debug)]
pub struct Runtime {
    system: system::Pallet<Self>,
    balances: balances::Pallet<Self>,
}

impl system::Config for Runtime {
    type AccountId = types::AccountId;
    type BlockNumber = types::BlockNumber;
    type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
    type Balance = types::Balance;
}

impl support::Dispatch for Runtime {
    type Caller = <Runtime as system::Config>::AccountId;
    type Call = RuntimeCall;
    type Error = balances::TransferError;

    fn dispatch(
        &mut self,
        caller: Self::Caller,
        runtime_call: Self::Call,
    ) -> support::DispatchResult<Self::Error> {
        match runtime_call {
            RuntimeCall::BalancesTransfer { to, amount } => {
                self.balances.transfer(&caller, &to, amount)?;
            }
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
        }
    }

    pub fn execute_block(
        &mut self,
        block: types::Block,
    ) -> support::DispatchResult<system::SystemError> {
        self.system.inc_block_number()?;

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
    runtime.balances.set_balance(&alice, 100);

    // Tx #1
    let block = types::Block {
        header: support::Header {
            block_number: runtime.system.block_number(),
        },
        extrinsics: vec![types::Extrinsic {
            caller: alice.clone(),
            call: RuntimeCall::BalancesTransfer {
                to: bob.clone(),
                amount: 100,
            },
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
            call: RuntimeCall::BalancesTransfer {
                to: bob.clone(),
                amount: 100,
            },
        }],
    };
    runtime.execute_block(block).expect("Invalid block");

    // Tx #3
    let block = types::Block {
        header: support::Header {
            block_number: runtime.system.block_number(),
        },
        extrinsics: vec![types::Extrinsic {
            caller: alice.clone(),
            call: RuntimeCall::BalancesTransfer {
                to: bob.clone(),
                amount: 100,
            },
        }],
    };
    runtime.execute_block(block).expect("Invalid block");

    println!("Runtime state: {:#?}", runtime);
}
