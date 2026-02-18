mod balances;
mod system;
mod types;

#[derive(Debug)]
pub struct Runtime {
    system: system::Pallet<Self>,
    balances: balances::Pallet<types::AccountId, types::Balance>,
}

impl system::Config for Runtime {
    type AccountId = String;
    type BlockNumber = u32;
    type Nonce = u32;
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
