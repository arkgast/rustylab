mod balances;
mod support;
mod system;

mod types {
    pub type AccountId = String;
    pub type Balance = u128;
    pub type BlockNumber = u32;
    pub type Nonce = u32;
    pub type Extrinsic = crate::support::Extrinsic<AccountId, crate::RuntimeCall>;
    pub type Header = crate::support::Header<BlockNumber>;
    pub type Block = crate::support::Block<Header, Extrinsic>;
}

pub enum RuntimeCall {}

#[derive(Debug)]
pub struct Runtime {
    system: system::Pallet<Self>,
    balances: balances::Pallet<Self>,
}

impl system::Config for Runtime {
    type AccountId = crate::types::AccountId;
    type BlockNumber = crate::types::BlockNumber;
    type Nonce = crate::types::Nonce;
}

impl balances::Config for Runtime {
    type Balance = crate::types::Balance;
}

#[allow(clippy::new_without_default)]
impl Runtime {
    pub fn new() -> Self {
        Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
        }
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
    assert_eq!(runtime.system.block_number(), 1);

    // first transaction
    runtime.system.inc_nonce(&"alice".to_string()).unwrap();
    assert_eq!(runtime.system.nonce(&alice), 1);
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
