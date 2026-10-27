//! Runner for S1-T3: `scripts/run.sh S1-T3`. Add your own `show!` lines freely.
use tasks::s01::t03_ownership::*;
use tasks::{lab::kata, show};

fn main() {
    kata("shout", || {
        let greeting = String::from("hello");
        let loud = shout(greeting);
        // `greeting` was moved into `shout`; using it here would not compile.
        show!(loud);
    });
    kata("longer", || {
        show!(longer(String::from("borrow"), String::from("move")));
    });
    kata("Wallet", || {
        let mut w = Wallet::new(String::from("alice"));
        for coin in [25, 100, 5] {
            w.deposit(coin);
        }
        show!(w);
        show!(w.total());
        show!(w.take_largest());
        show!(w);
        show!(w.into_owner());
    });
    kata("drain_into", || {
        let mut a = Wallet::new(String::from("a"));
        let mut b = Wallet::new(String::from("b"));
        a.deposit(1);
        a.deposit(2);
        b.deposit(9);
        a.drain_into(&mut b);
        show!(a);
        show!(b);
    });
    kata("swap_owners", || {
        let mut a = Wallet::new(String::from("alice"));
        let mut b = Wallet::new(String::from("bob"));
        swap_owners(&mut a, &mut b);
        show!((a.owner, b.owner));
    });
}
