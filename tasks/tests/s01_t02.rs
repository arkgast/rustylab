//! Gate for S1-T2. Do not edit.
mod common;

use tasks::s01::t02_structs_enums::*;

const SRC: &str = include_str!("../src/s01/t02_structs_enums.rs");

#[test]
fn rect_basics() {
    let r = Rect::new(3, 4);
    assert_eq!(
        r,
        Rect {
            width: 3,
            height: 4
        }
    );
    assert_eq!(r.area(), 12);
    assert_eq!(Rect::square(5), Rect::new(5, 5));
}

#[test]
fn rect_area_does_not_overflow() {
    let big = Rect::square(u32::MAX);
    assert_eq!(big.area(), 18_446_744_065_119_617_025);
}

#[test]
fn rect_can_hold() {
    let outer = Rect::new(10, 5);
    assert!(outer.can_hold(&Rect::new(10, 5)));
    assert!(outer.can_hold(&Rect::new(3, 2)));
    assert!(!outer.can_hold(&Rect::new(5, 10)), "no rotation");
    assert!(!outer.can_hold(&Rect::new(11, 1)));
}

#[test]
fn directions() {
    use Direction::*;
    assert_eq!(North.turn_right(), East);
    assert_eq!(East.turn_right(), South);
    assert_eq!(South.turn_right(), West);
    assert_eq!(West.turn_right(), North);
    for d in [North, East, South, West] {
        assert_eq!(d.turn_right().turn_right(), d.opposite());
        assert_eq!(d.opposite().opposite(), d);
    }
}

#[test]
fn position_steps_and_saturates() {
    let mut p = Position::default();
    p.step(Direction::North, 3);
    p.step(Direction::East, 2);
    p.step(Direction::South, 5);
    p.step(Direction::West, 1);
    assert_eq!(p, Position { x: 1, y: -2 });

    let mut edge = Position {
        x: i32::MAX - 1,
        y: i32::MIN + 1,
    };
    edge.step(Direction::East, 10);
    edge.step(Direction::South, 10);
    assert_eq!(
        edge,
        Position {
            x: i32::MAX,
            y: i32::MIN
        }
    );
}

#[test]
fn coins() {
    use Coin::*;
    let all = [Penny, Nickel, Dime, Quarter];
    let cents: Vec<u32> = all.iter().map(|c| c.value_in_cents()).collect();
    assert_eq!(cents, [1, 5, 10, 25]);
    for c in all {
        assert_eq!(Coin::from_cents(c.value_in_cents()), Some(c));
    }
    assert_eq!(Coin::from_cents(0), None);
    assert_eq!(Coin::from_cents(50), None);
}

#[test]
fn describe_commands() {
    let mv = Command::Move {
        dir: Direction::North,
        steps: 3,
    };
    assert_eq!(mv.describe(), "move North 3");
    assert_eq!(
        Command::Say("hi there".to_string()).describe(),
        "say \"hi there\""
    );
    assert_eq!(Command::Quit.describe(), "quit");
}

#[test]
fn rule_no_wildcard_arms() {
    common::assert_absent(
        SRC,
        "_ =>",
        "list every variant so new ones break the build",
    );
}
