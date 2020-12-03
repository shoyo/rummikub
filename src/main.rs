/// Copyright (c) 2020, Shoyo Inokuchi
use std::collections::HashMap;
use std::fmt;

/// A single Rummikub tile
#[derive(Debug, PartialEq)]
enum Tile {
    Basic(BasicTile),
    Joker(Joker),
}

#[derive(Debug, PartialEq)]
struct BasicTile {
    color: TileColor,
    value: TileValue,
}

impl BasicTile {
    fn new(color: TileColor, value: TileValue) -> Self {
        if value == 0 || value > 13 {
            panic!("Attempted to create a tile with an invalid value {}", value);
        }
        Self { color, value }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum TileColor {
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

type TileValue = u8;

#[derive(Debug, PartialEq)]
struct Joker {
    variant: JokerVariant,
}

impl Joker {
    fn new(variant: JokerVariant) -> Self {
        Self { variant }
    }
}

#[derive(Debug, PartialEq)]
enum JokerVariant {
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

enum Parsing {
    Run {
        /// `last_value` is unknown while the tiles encountered so far did not contain a basic tile, and
        /// included a color-change joker.
        ///
        /// Note: the parser is in the `Run` state because color-change jokers can only exist in a
        /// run or invalid sequence.
        last_value: Option<TileValue>,

        /// `allow` keeps track of which colors are allowed for the current tile.
        /// Typically, only one color maps to true while the rest map to false.
        /// However, when a color-change joker is encountered, the map inverts and the
        /// previous tile's color exclusively maps to false.
        /// In the case that a color-change joker is encountered after another color-change joker
        /// before encountering a basic tile, every color maps to true until a basic tile is encountered.
        allow: HashMap<TileColor, bool>,

        /// `size` tracks the current length of the sequence.
        size: u8,
    },
    Group {
        value: TileValue,
        allow: HashMap<TileColor, bool>,

        /// `size` tracks the current length of the sequence.
        size: u8,
    },
    Undetermined {
        /// `tile_seen` stores the first basic tile in the sequence. Its value is None until a basic
        /// tile is encountered.
        /// After a second basic tile is encountered, `tile_seen` is used to determined whether the
        /// sequence is a run or group, or is invalid.
        /// Any sequence can be definitely identified as a run, group, or invalid with two basic
        /// tiles.
        tile_seen: Option<BasicTile>,

        /// `size` tracks the current length of the sequence.
        size: u8,
    },
}

/// Given an ordered set of Rummikub tiles, return whether the set is valid.
fn is_valid_set(set: &Vec<Tile>) -> bool {
    if set.len() < 3 {
        return false;
    }
    let mut parsing = Parsing::Undetermined {
        tile_seen: None,
        size: 0,
    };
    let mut tiles = set.iter().enumerate();

    while let Some((index, tile)) = tiles.next() {
        match parsing {
            Parsing::Run {
                ref mut last_value,
                ref mut allow,
                ref mut size,
            } => match tile {
                Tile::Basic(t) => {
                    _assert_valid_tile_value(t.value);
                    if !allow[&t.color] {
                        return false;
                    }
                    if t.value <= *size {
                        return false;
                    }
                    *size += 1;
                    match last_value {
                        Some(ref mut val) => {
                            *val += 1;
                            if t.value != *val {
                                return false;
                            }
                        }
                        None => {
                            allow.insert(TileColor::Black, false);
                            allow.insert(TileColor::Red, false);
                            allow.insert(TileColor::Blue, false);
                            allow.insert(TileColor::Orange, false);
                            allow.insert(t.color, true);
                            *last_value = Some(t.value);
                        }
                    }
                }
                Tile::Joker(j) => match j.variant {
                    JokerVariant::Single => {
                        *size += 1;
                        match last_value {
                            Some(ref mut val) => {
                                *val += 1;
                                if *val > 13 {
                                    return false;
                                }
                            }
                            None => continue,
                        }
                    }
                    JokerVariant::Double => {
                        *size += 2;
                        match last_value {
                            Some(ref mut val) => {
                                *val += 2;
                                if *val > 13 {
                                    return false;
                                }
                            }
                            None => continue,
                        }
                    }
                    JokerVariant::Mirror => {
                        return _is_symmetric(&set, index);
                    }
                    JokerVariant::ColorChange => {
                        *size += 1;
                        match last_value {
                            Some(ref mut val) => {
                                *val += 1;
                                if *val > 13 {
                                    return false;
                                }

                                let cnt = allow.iter().map(|(_, v)| *v).len();
                                if cnt != 1 {
                                    panic!("Only one color should be allowed during a run with a known last value.");
                                }
                                for perm in allow.values_mut() {
                                    if *perm {
                                        *perm = false;
                                    } else {
                                        *perm = true;
                                    }
                                }
                            }
                            None => {
                                allow.insert(TileColor::Black, true);
                                allow.insert(TileColor::Red, true);
                                allow.insert(TileColor::Blue, true);
                                allow.insert(TileColor::Orange, true);
                            }
                        }
                    }
                },
            },
            Parsing::Group {
                ref mut value,
                ref mut allow,
                ref mut size,
            } => match tile {
                Tile::Basic(t) => {
                    _assert_valid_tile_value(t.value);
                }
                Tile::Joker(j) => match j.variant {
                    JokerVariant::Single => {
                        *size += 1;
                        if *size > 4 {
                            return false;
                        }
                    }
                    JokerVariant::Double => {
                        let num_allow = allow
                            .iter()
                            .filter(|(_, v)| **v)
                            .collect::<Vec<(&TileColor, &bool)>>()
                            .len();
                        if num_allow < 2 {
                            return false;
                        }
                        *size += 2;
                        if *size > 4 {
                            return false;
                        }
                    }
                    JokerVariant::Mirror => {
                        return _is_symmetric(&set, index);
                    }
                    JokerVariant::ColorChange => {
                        return false;
                    }
                },
            },
            Parsing::Undetermined {
                ref mut tile_seen,
                ref mut size,
            } => match tile {
                Tile::Basic(t) => {
                    _assert_valid_tile_value(t.value);
                    match tile_seen {
                        Some(ts) => {
                            if t.value == ts.value + 1 && t.color == ts.color {
                                // Check that the starting value of the run is valid.
                                // Ex. J J 3 4 .. is valid
                                //     J J 2 3 .. is NOT valid
                                if *size > ts.value {
                                    return false;
                                }

                                // Tile sequence is confirmed to be a run, so change the parser
                                // state.
                                let mut allow = HashMap::new();
                                allow.insert(TileColor::Black, false);
                                allow.insert(TileColor::Red, false);
                                allow.insert(TileColor::Blue, false);
                                allow.insert(TileColor::Orange, false);
                                allow.insert(ts.color, true);

                                parsing = Parsing::Run {
                                    last_value: Some(t.value),
                                    allow: allow,
                                    size: *size + 1,
                                };
                            } else if t.value == ts.value && t.color != ts.color {
                                // Check that the length of the group is valid.
                                // Ex. J J Red Blue   .. is valid
                                //     J DJ Red Blue .. is NOT valid
                                if *size > 3 {
                                    return false;
                                }

                                // Tile sequence is confirmed to be a group, so change the parser
                                // state.
                                let mut allow = HashMap::new();
                                allow.insert(TileColor::Black, true);
                                allow.insert(TileColor::Red, true);
                                allow.insert(TileColor::Blue, true);
                                allow.insert(TileColor::Orange, true);
                                allow.insert(t.color, false);
                                allow.insert(ts.color, false);

                                parsing = Parsing::Group {
                                    value: ts.value,
                                    allow: allow,
                                    size: *size + 1,
                                };
                            } else {
                                return false;
                            }
                        }
                        None => {
                            // Check that the current sequence is not an invalid run.
                            // (Group can be ruled out due to the total length being >= 5.)
                            // Ex. J J DJ 5 .. is valid
                            //     J J DJ 4 .. is NOT valid
                            if *size >= 4 && t.value <= *size {
                                return false;
                            }

                            // Update parser state.
                            *tile_seen = Some(BasicTile::new(t.color, t.value));
                            *size += 1;
                        }
                    }
                }
                Tile::Joker(j) => match j.variant {
                    JokerVariant::Single => {
                        *size += 1;
                    }
                    JokerVariant::Double => {
                        *size += 2;
                    }
                    JokerVariant::Mirror => {
                        _is_symmetric(&set, index);
                    }
                    JokerVariant::ColorChange => match tile_seen {
                        Some(ts) => {
                            let mut allow = HashMap::new();
                            allow.insert(TileColor::Black, true);
                            allow.insert(TileColor::Red, true);
                            allow.insert(TileColor::Blue, true);
                            allow.insert(TileColor::Orange, true);
                            allow.insert(ts.color, false);

                            parsing = Parsing::Run {
                                last_value: Some(ts.value + 1),
                                allow: allow,
                                size: *size + 1,
                            };
                        }
                        None => {
                            let mut allow = HashMap::new();
                            allow.insert(TileColor::Black, true);
                            allow.insert(TileColor::Red, true);
                            allow.insert(TileColor::Blue, true);
                            allow.insert(TileColor::Orange, true);

                            parsing = Parsing::Run {
                                last_value: None,
                                allow: allow,
                                size: *size + 1,
                            }
                        }
                    },
                },
            },
        }
    }
    return true;
}

fn _assert_valid_tile_value(value: TileValue) {
    if value == 0 || value > 13 {
        panic!(
            "Illegal tile value {}: tiles should be 0 < value <= 13",
            value
        );
    }
}

fn _is_symmetric(set: &Vec<Tile>, mirror_index: usize) -> bool {
    if set.len() < 3 {
        return false;
    }
    let mut left = mirror_index - 1;
    let mut right = mirror_index + 1;

    // Record-keeping flag for incrementing pointers to double jokers.
    let mut incr_double_joker = false;

    while left >= 0 && right < set.len() {
        match &set[left] {
            Tile::Basic(bl) => match &set[right] {
                Tile::Basic(br) => {
                    if bl != br {
                        return false;
                    }
                    left -= 1;
                    right += 1;
                }
                Tile::Joker(jr) => match jr.variant {
                    JokerVariant::Single => {
                        left -= 1;
                        right += 1;
                    }
                    JokerVariant::Double => {
                        if incr_double_joker {
                            right += 1;
                            incr_double_joker = false;
                        } else {
                            incr_double_joker = true;
                        }
                        left -= 1;
                    }
                    JokerVariant::Mirror => {
                        return false;
                    }
                    JokerVariant::ColorChange => {
                        return false;
                    }
                },
            },
            Tile::Joker(jl) => match &set[right] {
                Tile::Basic(_) => match jl.variant {
                    JokerVariant::Single => {
                        left -= 1;
                        right += 1;
                    }
                    JokerVariant::Double => {
                        if incr_double_joker {
                            left -= 1;
                            incr_double_joker = false;
                        } else {
                            incr_double_joker = true;
                        }
                        right += 1;
                    }
                    JokerVariant::Mirror => {
                        return false;
                    }
                    JokerVariant::ColorChange => {
                        return false;
                    }
                },
                Tile::Joker(jr) => match jl.variant {
                    JokerVariant::Single => match jr.variant {
                        JokerVariant::Single => {
                            left -= 1;
                            right += 1;
                        }
                        JokerVariant::Double => {
                            if incr_double_joker {
                                right += 1;
                                incr_double_joker = false;
                            } else {
                                incr_double_joker = true;
                            }
                            left -= 1;
                        }
                        JokerVariant::Mirror => {
                            return false;
                        }
                        JokerVariant::ColorChange => {
                            return false;
                        }
                    },
                    JokerVariant::Double => match jr.variant {
                        JokerVariant::Single => {
                            if incr_double_joker {
                                left -= 1;
                                incr_double_joker = false;
                            } else {
                                incr_double_joker = true;
                            }
                            right += 1;
                        }
                        JokerVariant::Double => {
                            left -= 1;
                            right += 1;
                        }
                        JokerVariant::Mirror => {
                            return false;
                        }
                        JokerVariant::ColorChange => {
                            return false;
                        }
                    },
                    JokerVariant::Mirror => {
                        return false;
                    }
                    JokerVariant::ColorChange => match jr.variant {
                        JokerVariant::Single => {
                            return false;
                        }
                        JokerVariant::Double => {
                            return false;
                        }
                        JokerVariant::Mirror => {
                            return false;
                        }
                        JokerVariant::ColorChange => {
                            left -= 1;
                            right += 1;
                        }
                    },
                },
            },
        }
    }
    if left != 0 || right != set.len() - 1 {
        panic!("ERROR: left and right indexes mismatched during traversal");
    }
    return true;
}

fn can_win(board: Vec<Vec<Tile>>, rack: Vec<Tile>) -> Result<(), ()> {
    Err(())
}

fn main() {
    let board = vec![vec![
        Tile::Basic(BasicTile::new(TileColor::Black, 2)),
        Tile::Basic(BasicTile::new(TileColor::Black, 3)),
        Tile::Basic(BasicTile::new(TileColor::Black, 4)),
    ]];
    let rack = vec![
        Tile::Basic(BasicTile::new(TileColor::Black, 10)),
        Tile::Basic(BasicTile::new(TileColor::Blue, 5)),
    ];

    match can_win(board, rack) {
        Ok(moves) => {
            println!("{:?}", moves);
        }
        Err(_) => println!("No winning move."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // BASIC RUNS

    #[test]
    fn test_valid_run() {
        let set = vec![
            Tile::Basic(BasicTile::new(TileColor::Red, 5)),
            Tile::Basic(BasicTile::new(TileColor::Red, 6)),
            Tile::Basic(BasicTile::new(TileColor::Red, 7)),
            Tile::Basic(BasicTile::new(TileColor::Red, 8)),
        ];
        assert_eq!(is_valid_set(&set), true);
    }

    #[test]
    fn test_invalid_descending_run() {
        let set = vec![
            Tile::Basic(BasicTile::new(TileColor::Blue, 9)),
            Tile::Basic(BasicTile::new(TileColor::Blue, 8)),
            Tile::Basic(BasicTile::new(TileColor::Blue, 7)),
        ];
        assert_eq!(is_valid_set(&set), false);
    }

    #[test]
    fn test_invalid_short_run() {
        let set = vec![
            Tile::Basic(BasicTile::new(TileColor::Blue, 7)),
            Tile::Basic(BasicTile::new(TileColor::Blue, 8)),
        ];
        assert_eq!(is_valid_set(&set), false);
    }

    // BASIC GROUPS

    #[test]
    fn test_valid_group_with_length_4() {
        let set = vec![
            Tile::Basic(BasicTile::new(TileColor::Red, 7)),
            Tile::Basic(BasicTile::new(TileColor::Blue, 7)),
            Tile::Basic(BasicTile::new(TileColor::Black, 7)),
            Tile::Basic(BasicTile::new(TileColor::Orange, 7)),
        ];
        assert_eq!(is_valid_set(&set), true);
    }

    #[test]
    fn test_valid_group_with_length_3() {
        let set = vec![
            Tile::Basic(BasicTile::new(TileColor::Red, 7)),
            Tile::Basic(BasicTile::new(TileColor::Black, 7)),
            Tile::Basic(BasicTile::new(TileColor::Orange, 7)),
        ];
        assert_eq!(is_valid_set(&set), true);
    }

    #[test]
    fn test_invalid_group_with_repeated_color() {
        let set = vec![
            Tile::Basic(BasicTile::new(TileColor::Red, 7)),
            Tile::Basic(BasicTile::new(TileColor::Red, 7)),
            Tile::Basic(BasicTile::new(TileColor::Black, 7)),
            Tile::Basic(BasicTile::new(TileColor::Orange, 7)),
        ];
        assert_eq!(is_valid_set(&set), false);
    }

    #[test]
    fn test_invalid_group_run_hybrid() {
        let set = vec![
            Tile::Basic(BasicTile::new(TileColor::Red, 8)),
            Tile::Basic(BasicTile::new(TileColor::Orange, 8)),
            Tile::Basic(BasicTile::new(TileColor::Blue, 8)),
            Tile::Basic(BasicTile::new(TileColor::Blue, 9)),
            Tile::Basic(BasicTile::new(TileColor::Blue, 10)),
        ];
        assert_eq!(is_valid_set(&set), false);
    }

    // SINGLE JOKER

    #[test]
    fn test_valid_run_with_single_joker() {
        let set = vec![
            Tile::Basic(BasicTile::new(TileColor::Red, 8)),
            Tile::Joker(Joker::new(JokerVariant::Single)),
            Tile::Basic(BasicTile::new(TileColor::Red, 10)),
        ];
        assert_eq!(is_valid_set(&set), true);
    }

    #[test]
    fn test_invalid_run_with_single_joker() {
        let set = vec![
            Tile::Basic(BasicTile::new(TileColor::Red, 8)),
            Tile::Joker(Joker::new(JokerVariant::Single)),
            Tile::Basic(BasicTile::new(TileColor::Red, 9)),
        ];
        assert_eq!(is_valid_set(&set), false);
    }

    #[test]
    fn test_invalid_run_with_single_joker_2() {
        let set = vec![
            Tile::Joker(Joker::new(JokerVariant::Single)),
            Tile::Basic(BasicTile::new(TileColor::Red, 1)),
            Tile::Basic(BasicTile::new(TileColor::Red, 2)),
            Tile::Basic(BasicTile::new(TileColor::Red, 3)),
        ];
        assert_eq!(is_valid_set(&set), false);
    }

    #[test]
    fn test_invalid_run_with_single_joker_3() {
        let set = vec![
            Tile::Basic(BasicTile::new(TileColor::Red, 11)),
            Tile::Basic(BasicTile::new(TileColor::Red, 12)),
            Tile::Basic(BasicTile::new(TileColor::Red, 13)),
            Tile::Joker(Joker::new(JokerVariant::Single)),
        ];
        assert_eq!(is_valid_set(&set), false);
    }

    #[test]
    fn test_valid_group_with_single_joker() {
        let set = vec![
            Tile::Basic(BasicTile::new(TileColor::Red, 8)),
            Tile::Joker(Joker::new(JokerVariant::Single)),
            Tile::Basic(BasicTile::new(TileColor::Blue, 8)),
        ];
        assert_eq!(is_valid_set(&set), true);
    }

    #[test]
    fn test_valid_run_with_two_single_jokers() {
        let set = vec![
            Tile::Joker(Joker::new(JokerVariant::Single)),
            Tile::Basic(BasicTile::new(TileColor::Blue, 8)),
            Tile::Joker(Joker::new(JokerVariant::Single)),
        ];
        assert_eq!(is_valid_set(&set), true);
    }

    #[test]
    fn test_invalid_run_with_more_single_jokers_than_in_the_box() {
        let set = vec![
            Tile::Joker(Joker::new(JokerVariant::Single)),
            Tile::Joker(Joker::new(JokerVariant::Single)),
            Tile::Basic(BasicTile::new(TileColor::Blue, 8)),
            Tile::Joker(Joker::new(JokerVariant::Single)),
        ];
        assert_eq!(is_valid_set(&set), false);
    }

    // DOUBLE JOKER

    #[test]
    fn test_valid_run_with_double_joker() {
        let set = vec![
            Tile::Joker(Joker::new(JokerVariant::Double)),
            Tile::Basic(BasicTile::new(TileColor::Blue, 8)),
            Tile::Basic(BasicTile::new(TileColor::Blue, 9)),
        ];
        assert_eq!(is_valid_set(&set), true);
    }

    #[test]
    fn test_invalid_run_with_double_joker() {
        let set = vec![
            Tile::Basic(BasicTile::new(TileColor::Blue, 7)),
            Tile::Joker(Joker::new(JokerVariant::Double)),
        ];
        assert_eq!(is_valid_set(&set), false);
    }

    #[test]
    fn test_valid_run_with_two_double_jokers() {
        let set = vec![
            Tile::Basic(BasicTile::new(TileColor::Blue, 3)),
            Tile::Joker(Joker::new(JokerVariant::Double)),
            Tile::Joker(Joker::new(JokerVariant::Double)),
            Tile::Basic(BasicTile::new(TileColor::Blue, 8)),
        ];
        assert_eq!(is_valid_set(&set), true);
    }

    #[test]
    fn test_valid_run_with_double_joker_2() {
        let set = vec![
            Tile::Basic(BasicTile::new(TileColor::Blue, 8)),
            Tile::Joker(Joker::new(JokerVariant::Double)),
            Tile::Basic(BasicTile::new(TileColor::Blue, 11)),
        ];
        assert_eq!(is_valid_set(&set), true);
    }

    #[test]
    fn test_invalid_run_with_double_joker_2() {
        let set = vec![
            Tile::Basic(BasicTile::new(TileColor::Blue, 8)),
            Tile::Joker(Joker::new(JokerVariant::Double)),
            Tile::Basic(BasicTile::new(TileColor::Red, 11)),
        ];
        assert_eq!(is_valid_set(&set), false);
    }

    #[test]
    fn test_invalid_group_with_double_joker() {
        let set = vec![
            Tile::Basic(BasicTile::new(TileColor::Red, 8)),
            Tile::Basic(BasicTile::new(TileColor::Blue, 8)),
            Tile::Joker(Joker::new(JokerVariant::Double)),
            Tile::Basic(BasicTile::new(TileColor::Black, 8)),
        ];
        assert_eq!(is_valid_set(&set), false);
    }

    // MIRROR JOKER

    #[test]
    fn test_valid_group_with_mirror() {
        let set = vec![
            Tile::Basic(BasicTile::new(TileColor::Black, 7)),
            Tile::Basic(BasicTile::new(TileColor::Red, 7)),
            Tile::Joker(Joker::new(JokerVariant::Mirror)),
            Tile::Basic(BasicTile::new(TileColor::Red, 7)),
            Tile::Basic(BasicTile::new(TileColor::Black, 7)),
        ];
        assert_eq!(is_valid_set(&set), true);
    }

    #[test]
    fn test_valid_run_with_mirror() {
        let set = vec![
            Tile::Basic(BasicTile::new(TileColor::Black, 7)),
            Tile::Basic(BasicTile::new(TileColor::Black, 8)),
            Tile::Joker(Joker::new(JokerVariant::Mirror)),
            Tile::Basic(BasicTile::new(TileColor::Black, 8)),
            Tile::Basic(BasicTile::new(TileColor::Black, 7)),
        ];
        assert_eq!(is_valid_set(&set), true);
    }

    #[test]
    fn test_invalid_group_with_mirror() {
        let set = vec![
            Tile::Basic(BasicTile::new(TileColor::Red, 7)),
            Tile::Basic(BasicTile::new(TileColor::Black, 7)),
            Tile::Joker(Joker::new(JokerVariant::Mirror)),
            Tile::Basic(BasicTile::new(TileColor::Red, 7)),
            Tile::Basic(BasicTile::new(TileColor::Black, 7)),
        ];
        assert_eq!(is_valid_set(&set), false);
    }

    #[test]
    fn test_invalid_run_with_mirror() {
        let set = vec![
            Tile::Basic(BasicTile::new(TileColor::Red, 8)),
            Tile::Basic(BasicTile::new(TileColor::Red, 7)),
            Tile::Joker(Joker::new(JokerVariant::Mirror)),
            Tile::Basic(BasicTile::new(TileColor::Red, 7)),
            Tile::Basic(BasicTile::new(TileColor::Red, 8)),
        ];
        assert_eq!(is_valid_set(&set), false);
    }

    #[test]
    fn test_invalid_run_with_mirror_2() {
        let set = vec![
            Tile::Basic(BasicTile::new(TileColor::Red, 7)),
            Tile::Basic(BasicTile::new(TileColor::Red, 8)),
            Tile::Joker(Joker::new(JokerVariant::Mirror)),
            Tile::Basic(BasicTile::new(TileColor::Red, 8)),
            Tile::Basic(BasicTile::new(TileColor::Red, 7)),
            Tile::Basic(BasicTile::new(TileColor::Red, 6)),
        ];
        assert_eq!(is_valid_set(&set), false);
    }

    #[test]
    fn test_invalid_run_with_two_mirrors() {
        let set = vec![
            Tile::Basic(BasicTile::new(TileColor::Red, 8)),
            Tile::Joker(Joker::new(JokerVariant::Mirror)),
            Tile::Basic(BasicTile::new(TileColor::Red, 8)),
            Tile::Joker(Joker::new(JokerVariant::Mirror)),
            Tile::Basic(BasicTile::new(TileColor::Red, 8)),
        ];
        assert_eq!(is_valid_set(&set), false);
    }

    // COLOR CHANGE JOKER

    #[test]
    fn test_valid_run_with_color_change() {
        let set = vec![
            Tile::Basic(BasicTile::new(TileColor::Red, 6)),
            Tile::Basic(BasicTile::new(TileColor::Red, 7)),
            Tile::Joker(Joker::new(JokerVariant::ColorChange)),
            Tile::Basic(BasicTile::new(TileColor::Blue, 9)),
            Tile::Basic(BasicTile::new(TileColor::Blue, 10)),
        ];
        assert_eq!(is_valid_set(&set), true);
    }

    #[test]
    fn test_invalid_run_with_color_change() {
        let set = vec![
            Tile::Basic(BasicTile::new(TileColor::Red, 6)),
            Tile::Basic(BasicTile::new(TileColor::Red, 7)),
            Tile::Joker(Joker::new(JokerVariant::ColorChange)),
            Tile::Basic(BasicTile::new(TileColor::Red, 9)),
            Tile::Basic(BasicTile::new(TileColor::Red, 10)),
        ];
        assert_eq!(is_valid_set(&set), false);
    }

    #[test]
    fn test_invalid_run_with_color_change_without_number_skip() {
        let set = vec![
            Tile::Basic(BasicTile::new(TileColor::Red, 6)),
            Tile::Basic(BasicTile::new(TileColor::Red, 7)),
            Tile::Joker(Joker::new(JokerVariant::ColorChange)),
            Tile::Basic(BasicTile::new(TileColor::Red, 8)),
            Tile::Basic(BasicTile::new(TileColor::Red, 9)),
        ];
        assert_eq!(is_valid_set(&set), false);
    }

    #[test]
    fn test_valid_run_with_two_color_changes() {
        let set = vec![
            Tile::Basic(BasicTile::new(TileColor::Orange, 6)),
            Tile::Joker(Joker::new(JokerVariant::ColorChange)),
            Tile::Basic(BasicTile::new(TileColor::Red, 8)),
            Tile::Joker(Joker::new(JokerVariant::ColorChange)),
            Tile::Basic(BasicTile::new(TileColor::Blue, 10)),
        ];
        assert_eq!(is_valid_set(&set), true);
    }

    #[test]
    fn test_valid_run_with_two_color_change_sandwich() {
        let set = vec![
            Tile::Joker(Joker::new(JokerVariant::ColorChange)),
            Tile::Basic(BasicTile::new(TileColor::Red, 8)),
            Tile::Joker(Joker::new(JokerVariant::ColorChange)),
        ];
        assert_eq!(is_valid_set(&set), true);
    }

    #[test]
    fn test_valid_run_with_two_color_change_adjacent() {
        let set = vec![
            Tile::Basic(BasicTile::new(TileColor::Orange, 6)),
            Tile::Joker(Joker::new(JokerVariant::ColorChange)),
            Tile::Joker(Joker::new(JokerVariant::ColorChange)),
        ];
        assert_eq!(is_valid_set(&set), true);
    }

    #[test]
    fn test_adjacent_color_change_house_rule() {
        let set = vec![
            Tile::Basic(BasicTile::new(TileColor::Orange, 6)),
            Tile::Joker(Joker::new(JokerVariant::ColorChange)),
            Tile::Joker(Joker::new(JokerVariant::ColorChange)),
            Tile::Basic(BasicTile::new(TileColor::Blue, 9)),
        ];
        assert_eq!(is_valid_set(&set), true);
    }

    #[test]
    fn test_adjacent_color_change_house_rule_2() {
        let set = vec![
            Tile::Basic(BasicTile::new(TileColor::Orange, 6)),
            Tile::Joker(Joker::new(JokerVariant::ColorChange)),
            Tile::Joker(Joker::new(JokerVariant::ColorChange)),
            Tile::Basic(BasicTile::new(TileColor::Orange, 9)),
        ];
        assert_eq!(is_valid_set(&set), true);
    }

    // MIXED JOKERS

    #[test]
    fn test_valid_mixed_1() {
        let set = vec![
            Tile::Basic(BasicTile::new(TileColor::Red, 6)),
            Tile::Basic(BasicTile::new(TileColor::Red, 7)),
            Tile::Basic(BasicTile::new(TileColor::Red, 8)),
            Tile::Joker(Joker::new(JokerVariant::Mirror)),
            Tile::Basic(BasicTile::new(TileColor::Red, 8)),
            Tile::Joker(Joker::new(JokerVariant::Single)),
            Tile::Basic(BasicTile::new(TileColor::Red, 6)),
        ];
        assert_eq!(is_valid_set(&set), true);
    }

    #[test]
    fn test_valid_mixed_2() {
        let set = vec![
            Tile::Basic(BasicTile::new(TileColor::Blue, 6)),
            Tile::Joker(Joker::new(JokerVariant::ColorChange)),
            Tile::Basic(BasicTile::new(TileColor::Red, 8)),
            Tile::Joker(Joker::new(JokerVariant::Mirror)),
            Tile::Basic(BasicTile::new(TileColor::Red, 8)),
            Tile::Joker(Joker::new(JokerVariant::ColorChange)),
            Tile::Basic(BasicTile::new(TileColor::Blue, 6)),
        ];
        assert_eq!(is_valid_set(&set), true);
    }

    #[test]
    fn test_valid_mixed_3() {
        let set = vec![
            Tile::Basic(BasicTile::new(TileColor::Red, 6)),
            Tile::Joker(Joker::new(JokerVariant::Single)),
            Tile::Basic(BasicTile::new(TileColor::Red, 8)),
            Tile::Joker(Joker::new(JokerVariant::Mirror)),
            Tile::Basic(BasicTile::new(TileColor::Red, 8)),
            Tile::Basic(BasicTile::new(TileColor::Red, 7)),
            Tile::Basic(BasicTile::new(TileColor::Red, 6)),
        ];
        assert_eq!(is_valid_set(&set), true);
    }

    #[test]
    fn test_valid_mixed_4() {
        let set = vec![
            Tile::Basic(BasicTile::new(TileColor::Red, 6)),
            Tile::Basic(BasicTile::new(TileColor::Red, 7)),
            Tile::Basic(BasicTile::new(TileColor::Red, 8)),
            Tile::Basic(BasicTile::new(TileColor::Red, 9)),
            Tile::Joker(Joker::new(JokerVariant::Mirror)),
            Tile::Joker(Joker::new(JokerVariant::Double)),
            Tile::Joker(Joker::new(JokerVariant::Double)),
        ];
        assert_eq!(is_valid_set(&set), true);
    }

    #[test]
    fn test_valid_mixed_5() {
        let set = vec![
            Tile::Joker(Joker::new(JokerVariant::Double)),
            Tile::Joker(Joker::new(JokerVariant::ColorChange)),
            Tile::Joker(Joker::new(JokerVariant::Double)),
            Tile::Basic(BasicTile::new(TileColor::Red, 3)),
        ];
        assert_eq!(is_valid_set(&set), false);
    }
}
