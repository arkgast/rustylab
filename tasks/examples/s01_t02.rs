//! Runner for S1-T2: `scripts/run.sh S1-T2`. Add your own `show!` lines freely.
use tasks::s01::t02_structs_enums::*;
use tasks::{lab::kata, show};

fn main() {
    kata("Rect", || {
        let r = Rect::new(10, 5);
        show!(r);
        show!(r.area());
        show!(r.can_hold(&Rect::square(4)));
        show!(r.can_hold(&Rect::new(5, 10)));
        show!(Rect::square(u32::MAX).area());
    });
    kata("Direction", || {
        let mut d = Direction::North;
        for _ in 0..4 {
            println!("  {d:?} -> turn_right -> {:?}", d.turn_right());
            d = d.turn_right();
        }
        show!(Direction::East.opposite());
    });
    kata("Position::step", || {
        let mut p = Position::default();
        for (dir, steps) in [
            (Direction::North, 3),
            (Direction::East, 2),
            (Direction::South, 5),
        ] {
            p.step(dir, steps);
            println!("  after {dir:?} {steps}: {p:?}");
        }
    });
    kata("Coin", || {
        show!(Coin::Quarter.value_in_cents());
        show!(Coin::from_cents(10));
        show!(Coin::from_cents(3));
    });
    kata("Command::describe", || {
        let commands = [
            Command::Move {
                dir: Direction::West,
                steps: 2,
            },
            Command::Say("hello".to_string()),
            Command::Quit,
        ];
        for c in &commands {
            println!("  {c:?} => {}", c.describe());
        }
    });
}
