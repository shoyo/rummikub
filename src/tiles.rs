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

/// Utilities

/// Convert a string containing space-limited tile abbreviations (such as r5 - Red 5 tile, j - Single
/// Joker tile, etc.) and return a vector of the corresponding set.
///
/// Abbreviations:
///
/// Basic tiles: <tile color><tile value>
///     Red    --> "r"
///     Orange --> "o"
///     Black  --> "a"
///     Blue   --> "u"
///
/// Jokers: <joker type>
///     Single Joker      --> "j"
///     Double Joker      --> "d"
///     Mirror Joker      --> "m"
///     ColorChange Joker --> "c"
///
/// Examples:
///     "r1 r2 r3"
///     "a6 c u8 u9 m j u8 c a6"
pub fn deserialize_set(input: String) -> Result<Vec<Tile>, String> {
    let mut stream = input.split(' ');
    let mut vec = Vec::new();
    while let Some(token) = stream.next() {
        match token.chars().nth(0).unwrap() {
            'r' => {
                let val = match parse_tile_value(&token[1..]) {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                };
                vec.push(Tile::Basic(BasicTile::new(TileColor::Red, val)));
            }
            'o' => {
                let val = match parse_tile_value(&token[1..]) {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                };
                vec.push(Tile::Basic(BasicTile::new(TileColor::Orange, val)));
            }
            'u' => {
                let val = match parse_tile_value(&token[1..]) {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                };
                vec.push(Tile::Basic(BasicTile::new(TileColor::Blue, val)));
            }
            'a' => {
                let val = match parse_tile_value(&token[1..]) {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                };
                vec.push(Tile::Basic(BasicTile::new(TileColor::Black, val)));
            }
            'j' => {
                if token.len() > 1 {
                    return Err(format!("Unrecognized token {}. Did you mean 'j'?", token));
                }
                vec.push(Tile::Joker(Joker::new(JokerVariant::Single)));
            }
            'd' => {
                if token.len() > 1 {
                    return Err(format!("Unrecognized token {}. Did you mean 'd'?", token));
                }
                vec.push(Tile::Joker(Joker::new(JokerVariant::Double)));
            }
            'm' => {
                if token.len() > 1 {
                    return Err(format!("Unrecognized token {}. Did you mean 'm'?", token));
                }
                vec.push(Tile::Joker(Joker::new(JokerVariant::Mirror)));
            }
            'c' => {
                if token.len() > 1 {
                    return Err(format!("Unrecognized token {}. Did you mean 'c'?", token));
                }
                vec.push(Tile::Joker(Joker::new(JokerVariant::ColorChange)));
            }
            _ => return Err(format!("Unrecognized token {}", token)),
        }
    }
    Ok(vec)
}

fn parse_tile_value(token: &str) -> Result<TileValue, String> {
    let val = match token.parse::<TileValue>() {
        Ok(v) => v,
        Err(_) => return Err(format!("Invalid tile value in token: {}", token)),
    };
    if val == 0 || val > 13 {
        return Err(format!("Invalid tile value {} in token: {}", val, token));
    }
    Ok(val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vectorize_set_1() {
        let input = "r1 r2 r3 j d r7".to_string();
        let expected = vec![
            Tile::Basic(BasicTile::new(TileColor::Red, 1)),
            Tile::Basic(BasicTile::new(TileColor::Red, 2)),
            Tile::Basic(BasicTile::new(TileColor::Red, 3)),
            Tile::Joker(Joker::new(JokerVariant::Single)),
            Tile::Joker(Joker::new(JokerVariant::Double)),
            Tile::Basic(BasicTile::new(TileColor::Red, 7)),
        ];
        assert_eq!(deserialize_set(input).unwrap(), expected);
    }

    #[test]
    fn test_vectorize_set_2() {
        let input = "a6 c u8 u9 m j u8 c a6".to_string();
        let expected = vec![
            Tile::Basic(BasicTile::new(TileColor::Black, 6)),
            Tile::Joker(Joker::new(JokerVariant::ColorChange)),
            Tile::Basic(BasicTile::new(TileColor::Blue, 8)),
            Tile::Basic(BasicTile::new(TileColor::Blue, 9)),
            Tile::Joker(Joker::new(JokerVariant::Mirror)),
            Tile::Joker(Joker::new(JokerVariant::Single)),
            Tile::Basic(BasicTile::new(TileColor::Blue, 8)),
            Tile::Joker(Joker::new(JokerVariant::ColorChange)),
            Tile::Basic(BasicTile::new(TileColor::Black, 6)),
        ];
        assert_eq!(deserialize_set(input).unwrap(), expected);
    }
}
