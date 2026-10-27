//! S1-T2 · Structs, methods & enums.
//!
//! Replace every `todo!()`. Run `scripts/run.sh S1-T2` to see your results,
//! `scripts/gate.sh S1-T2` to check them.
//!
//! Rule: no `_ =>` wildcard arm when matching on the enums of this file.

/// An axis-aligned rectangle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub width: u32,
    pub height: u32,
}

impl Rect {
    pub fn new(_width: u32, _height: u32) -> Self {
        todo!()
    }

    /// A square with sides of `size`.
    pub fn square(_size: u32) -> Self {
        todo!()
    }

    /// Area that cannot overflow, even for `u32::MAX` sides.
    pub fn area(&self) -> u64 {
        todo!()
    }

    /// `true` if `other` fits inside `self` without rotating it.
    pub fn can_hold(&self, _other: &Rect) -> bool {
        todo!()
    }
}

/// Compass direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    /// Rotates 90° clockwise: North → East → South → West → North.
    pub fn turn_right(self) -> Direction {
        todo!()
    }

    pub fn opposite(self) -> Direction {
        todo!()
    }
}

/// A point on a grid. North is +y, East is +x.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    /// Moves `steps` cells towards `dir`, in place. Saturates at the `i32` bounds
    /// instead of overflowing.
    pub fn step(&mut self, _dir: Direction, _steps: i32) {
        todo!()
    }
}

/// US coins.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

impl Coin {
    /// 1, 5, 10, 25.
    pub fn value_in_cents(self) -> u32 {
        todo!()
    }

    /// The coin worth exactly `cents`, if there is one.
    pub fn from_cents(_cents: u32) -> Option<Coin> {
        todo!()
    }
}

/// A command sent to a robot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Move { dir: Direction, steps: i32 },
    Say(String),
    Quit,
}

impl Command {
    /// Human-readable text:
    /// - `Move { dir: North, steps: 3 }` → `"move North 3"`
    /// - `Say("hi")` → `"say \"hi\""` (the text in double quotes)
    /// - `Quit` → `"quit"`
    pub fn describe(&self) -> String {
        todo!()
    }
}
