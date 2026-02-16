mod balances;
mod system;
mod types;

#[derive(Debug)]
pub struct Runtime<'a> {
    system: system::Pallet<'a>,
    balances: balances::Pallet<'a>,
}

#[allow(clippy::new_without_default)]
impl<'a> Runtime<'a> {
    pub fn new() -> Self {
        Self {
            system: system::Pallet::<'a>::new(),
            balances: balances::Pallet::new(),
        }
    }
}

fn main() {
    let mut runtime = Runtime::new();
    runtime.balances.set_balance("alice", 100);

    // simulate block
    runtime.system.inc_block_number();
    assert_eq!(runtime.system.block_number(), 1);

    // first transaction
    runtime.system.inc_nonce("alice");
    assert_eq!(runtime.system.nonce("alice"), 1);
    let transfer_result = runtime
        .balances
        .transfer("alice", "bob", 30)
        .inspect_err(|e| eprintln!("1st tx error: {:?}", e));
    println!("1st tx: {:?}", transfer_result);

    // second transaction
    runtime.system.inc_nonce("alice");
    let transfer_result = runtime
        .balances
        .transfer("alice", "bob", 80)
        .inspect_err(|e| eprintln!("2nd tx error: {:?}", e));
    println!("2nd tx: {:?}", transfer_result);

    println!("Runtime state: {:#?}", runtime);
}
