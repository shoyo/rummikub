/// Copyright (c) 2020, Shoyo Inokuchi
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Tile {
    Basic(BasicTile),
    Joker(Joker),
}

#[derive(Debug, PartialEq)]
pub struct BasicTile {
    pub color: TileColor,
    pub value: TileValue,
}

impl BasicTile {
    pub fn new(color: TileColor, value: TileValue) -> Self {
        if value == 0 || value > 13 {
            panic!("Attempted to create a tile with an invalid value {}", value);
        }
        Self { color, value }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum TileColor {
    Black,
    Red,
    Blue,
    Orange,
}

impl fmt::Display for TileColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TileColor::Black => write!(f, "BLACK"),
            TileColor::Red => write!(f, "RED"),
            TileColor::Blue => write!(f, "BLUE"),
            TileColor::Orange => write!(f, "ORANGE"),
        }
    }
}

pub type TileValue = u8;

#[derive(Debug, PartialEq)]
pub struct Joker {
    pub variant: JokerVariant,
}

impl Joker {
    pub fn new(variant: JokerVariant) -> Self {
        Self { variant }
    }
}

#[derive(Debug, PartialEq)]
pub enum JokerVariant {
    Single,
    Double,
    Mirror,
    ColorChange,
}

impl fmt::Display for JokerVariant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            JokerVariant::Single => write!(f, "SINGLE"),
            JokerVariant::Double => write!(f, "DOUBLE"),
            JokerVariant::Mirror => write!(f, "MIRROR"),
            JokerVariant::ColorChange => write!(f, "COLOR CHANGE"),
        }
    }
}
