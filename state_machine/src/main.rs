use crate::{balances::TransferError, support::Dispatch, system::SystemError};

mod balances;
mod support;
mod system;
mod types;

pub enum RuntimeCall {}

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
    type Caller = types::AccountId;
    type Call = RuntimeCall;

    fn dispatch<T>(
        &mut self,
        _caller: Self::Caller,
        _call: Self::Call,
    ) -> support::DispatchResult<T> {
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

    pub fn execute_block(&mut self, block: types::Block) -> support::DispatchResult<SystemError> {
        self.system.inc_block_number()?;

        for (idx, support::Extrinsic { caller, call }) in block.extrinsics.into_iter().enumerate() {
            self.system.inc_nonce(&caller)?;

            let res = self.dispatch::<TransferError>(caller, call).map_err(|e| {
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
    let mut runtime = Runtime::new();
    let alice = "alice".to_string();
    let bob = "bob".to_string();

    runtime.balances.set_balance(&alice, 100);
    runtime.balances.set_balance(&bob, 0);

    // simulate block
    runtime.system.inc_block_number().unwrap();

    // first transaction
    runtime.system.inc_nonce(&alice).unwrap();

    let transfer_result = runtime
        .balances
        .transfer(&"alice".to_string(), &bob, 30)
        .inspect_err(|e| eprintln!("1st tx error: {:?}", e));
    println!("1st tx: {:?}", transfer_result);

    // second transaction
    runtime.system.inc_nonce(&alice).unwrap();
    let transfer_result = runtime
        .balances
        .transfer(&alice, &bob, 80)
        .inspect_err(|e| eprintln!("2nd tx error: {:?}", e));
    println!("2nd tx: {:?}", transfer_result);

    println!("Runtime state: {:#?}", runtime);
}
