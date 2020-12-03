/// Copyright (c) 2020, Shoyo Inokuchi
use std::collections::HashMap;
use std::fmt;

/// A single Rummikub tile
#[derive(PartialEq)]
enum Tile {
    Basic(BasicTile),
    Joker(Joker),
}

#[derive(PartialEq)]
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

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
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

#[derive(PartialEq)]
struct Joker {
    variant: JokerVariant,
}

impl Joker {
    fn new(variant: JokerVariant) -> Self {
        Self { variant }
    }
}

#[derive(PartialEq)]
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
        last_value: TileValue,
        allowed: HashMap<TileColor, bool>,
    },
    Group {
        value: TileValue,
        seen: HashMap<TileColor, bool>,
        size: u8,
    },
    Undetermined {
        tile_seen: Option<(TileValue, TileColor)>,
        size: u8,
    },
}

/// Given an ordered set of Rummikub tiles, return whether the set is valid.
fn is_valid_set(set: Vec<Tile>) -> bool {
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
                ref mut allowed,
            } => match tile {
                Tile::Basic(t) => {
                    _assert_valid_tile_value(t.value);
                    if t.value != *last_value + 1 {
                        return false;
                    }
                    if !allowed[&t.color] {
                        return false;
                    }
                    *last_value += 1;
                }
                Tile::Joker(j) => match j.variant {
                    JokerVariant::Single => {
                        *last_value += 1;
                        if *last_value > 13 {
                            return false;
                        }
                    }
                    JokerVariant::Double => {
                        *last_value += 2;
                        if *last_value > 13 {
                            return false;
                        }
                    }
                    JokerVariant::Mirror => {
                        return _handle_mirror_joker(&set, index);
                    }
                    JokerVariant::ColorChange => {
                        *last_value += 1;
                        allowed.insert(TileColor::Black, true);
                        allowed.insert(TileColor::Red, true);
                        allowed.insert(TileColor::Blue, true);
                        allowed.insert(TileColor::Orange, true);
                    }
                },
            },
            Parsing::Group {
                ref mut value,
                ref mut seen,
                ref mut size,
            } => match tile {
                Tile::Basic(t) => {
                    _assert_valid_tile_value(t.value);
                }
                Tile::Joker(j) => match j.variant {
                    JokerVariant::Single => {
                        *size += 1;
                    }
                    JokerVariant::Double => {
                        let num_seen = seen
                            .iter()
                            .filter(|(_, v)| **v)
                            .collect::<Vec<(&TileColor, &bool)>>()
                            .len();
                        if num_seen > 2 {
                            return false;
                        }
                        *size += 2;
                    }
                    JokerVariant::Mirror => {
                        return _handle_mirror_joker(&set, index);
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
                    match *tile_seen {
                        Some((value, color)) => {
                            if t.value == value + 1 && t.color == color {
                                // Check that starting value of run is valid.
                                // Ex. J J 3 4 .. is valid
                                //     J J 2 3 .. is NOT valid
                                if *size > value {
                                    return false;
                                }

                                // Tile sequence is confirmed to be a run, so change the parser
                                // state.
                                let mut allowed = HashMap::new();
                                allowed.insert(TileColor::Black, false);
                                allowed.insert(TileColor::Red, false);
                                allowed.insert(TileColor::Blue, false);
                                allowed.insert(TileColor::Orange, false);
                                allowed.insert(color, true);

                                parsing = Parsing::Run {
                                    last_value: t.value,
                                    allowed: allowed,
                                };
                            } else if t.value == value && t.color != color {
                                // Check that length of group is valid.
                                // Ex. J J Red Blue   .. is valid
                                //     J J J Red Blue .. is NOT valid
                                if *size > 3 {
                                    return false;
                                }

                                // Tile sequence is confirmed to be a group, so change the parser
                                // state.
                                let mut seen = HashMap::new();
                                seen.insert(TileColor::Black, false);
                                seen.insert(TileColor::Red, false);
                                seen.insert(TileColor::Blue, false);
                                seen.insert(TileColor::Orange, false);
                                seen.insert(t.color, true);
                                seen.insert(color, true);

                                parsing = Parsing::Group {
                                    value: value,
                                    seen: seen,
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
                            *tile_seen = Some((t.value, t.color));
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
                        _handle_mirror_joker(&set, index);
                    }
                    JokerVariant::ColorChange => match tile_seen {
                        Some((value, color)) => {}
                        None => {}
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

fn _handle_mirror_joker(set: &Vec<Tile>, mirror_index: usize) -> bool {
    if mirror_index == 0 {
        return false;
    }
    let mut left = mirror_index - 1;
    let mut right = mirror_index + 1;
    while left >= 0 && right < set.len() {
        if left == 0 && right != set.len() - 1 || left != 0 && right == set.len() {
            return false;
        }
        if set[left] != set[right] {
            return false;
        }
        left -= 1;
        right += 1;
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
        assert_eq!(is_valid_set(set), true);
    }

    #[test]
    fn test_invalid_descending_run() {
        let set = vec![
            Tile::Basic(BasicTile::new(TileColor::Blue, 9)),
            Tile::Basic(BasicTile::new(TileColor::Blue, 8)),
            Tile::Basic(BasicTile::new(TileColor::Blue, 7)),
        ];
        assert_eq!(is_valid_set(set), false);
    }

    #[test]
    fn test_invalid_short_run() {
        let set = vec![
            Tile::Basic(BasicTile::new(TileColor::Blue, 7)),
            Tile::Basic(BasicTile::new(TileColor::Blue, 8)),
        ];
        assert_eq!(is_valid_set(set), false);
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
        assert_eq!(is_valid_set(set), true);
    }

    #[test]
    fn test_valid_group_with_length_3() {
        let set = vec![
            Tile::Basic(BasicTile::new(TileColor::Red, 7)),
            Tile::Basic(BasicTile::new(TileColor::Black, 7)),
            Tile::Basic(BasicTile::new(TileColor::Orange, 7)),
        ];
        assert_eq!(is_valid_set(set), true);
    }

    #[test]
    fn test_invalid_group_with_repeated_color() {
        let set = vec![
            Tile::Basic(BasicTile::new(TileColor::Red, 7)),
            Tile::Basic(BasicTile::new(TileColor::Red, 7)),
            Tile::Basic(BasicTile::new(TileColor::Black, 7)),
            Tile::Basic(BasicTile::new(TileColor::Orange, 7)),
        ];
        assert_eq!(is_valid_set(set), false);
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
        assert_eq!(is_valid_set(set), false);
    }

    // SINGLE JOKER

    #[test]
    fn test_valid_run_with_single_joker() {
        let set = vec![
            Tile::Basic(BasicTile::new(TileColor::Red, 8)),
            Tile::Joker(Joker::new(JokerVariant::Single)),
            Tile::Basic(BasicTile::new(TileColor::Red, 10)),
        ];
        assert_eq!(is_valid_set(set), true);
    }

    #[test]
    fn test_invalid_run_with_single_joker() {
        let set = vec![
            Tile::Basic(BasicTile::new(TileColor::Red, 8)),
            Tile::Joker(Joker::new(JokerVariant::Single)),
            Tile::Basic(BasicTile::new(TileColor::Red, 9)),
        ];
        assert_eq!(is_valid_set(set), false);
    }

    #[test]
    fn test_invalid_run_with_single_joker_2() {
        let set = vec![
            Tile::Joker(Joker::new(JokerVariant::Single)),
            Tile::Basic(BasicTile::new(TileColor::Red, 1)),
            Tile::Basic(BasicTile::new(TileColor::Red, 2)),
            Tile::Basic(BasicTile::new(TileColor::Red, 3)),
        ];
        assert_eq!(is_valid_set(set), false);
    }

    #[test]
    fn test_valid_group_with_single_joker() {
        let set = vec![
            Tile::Basic(BasicTile::new(TileColor::Red, 8)),
            Tile::Joker(Joker::new(JokerVariant::Single)),
            Tile::Basic(BasicTile::new(TileColor::Blue, 8)),
        ];
        assert_eq!(is_valid_set(set), true);
    }

    #[test]
    fn test_valid_run_with_two_single_jokers() {
        let set = vec![
            Tile::Joker(Joker::new(JokerVariant::Single)),
            Tile::Basic(BasicTile::new(TileColor::Blue, 8)),
            Tile::Joker(Joker::new(JokerVariant::Single)),
        ];
        assert_eq!(is_valid_set(set), true);
    }

    #[test]
    fn test_invalid_run_with_more_single_jokers_than_in_the_box() {
        let set = vec![
            Tile::Joker(Joker::new(JokerVariant::Single)),
            Tile::Joker(Joker::new(JokerVariant::Single)),
            Tile::Basic(BasicTile::new(TileColor::Blue, 8)),
            Tile::Joker(Joker::new(JokerVariant::Single)),
        ];
        assert_eq!(is_valid_set(set), false);
    }

    // DOUBLE JOKER

    #[test]
    fn test_valid_run_with_double_joker() {
        let set = vec![
            Tile::Joker(Joker::new(JokerVariant::Double)),
            Tile::Basic(BasicTile::new(TileColor::Blue, 8)),
            Tile::Basic(BasicTile::new(TileColor::Blue, 9)),
        ];
        assert_eq!(is_valid_set(set), true);
    }

    #[test]
    fn test_invalid_run_with_double_joker() {
        let set = vec![
            Tile::Basic(BasicTile::new(TileColor::Blue, 7)),
            Tile::Joker(Joker::new(JokerVariant::Double)),
        ];
        assert_eq!(is_valid_set(set), false);
    }

    #[test]
    fn test_valid_run_with_two_double_jokers() {
        let set = vec![
            Tile::Basic(BasicTile::new(TileColor::Blue, 3)),
            Tile::Joker(Joker::new(JokerVariant::Double)),
            Tile::Joker(Joker::new(JokerVariant::Double)),
            Tile::Basic(BasicTile::new(TileColor::Blue, 8)),
        ];
        assert_eq!(is_valid_set(set), true);
    }

    #[test]
    fn test_valid_run_with_double_joker_2() {
        let set = vec![
            Tile::Basic(BasicTile::new(TileColor::Blue, 8)),
            Tile::Joker(Joker::new(JokerVariant::Double)),
            Tile::Basic(BasicTile::new(TileColor::Blue, 11)),
        ];
        assert_eq!(is_valid_set(set), true);
    }

    #[test]
    fn test_invalid_run_with_double_joker_2() {
        let set = vec![
            Tile::Basic(BasicTile::new(TileColor::Blue, 8)),
            Tile::Joker(Joker::new(JokerVariant::Double)),
            Tile::Basic(BasicTile::new(TileColor::Red, 11)),
        ];
        assert_eq!(is_valid_set(set), false);
    }

    #[test]
    fn test_invalid_group_with_double_joker() {
        let set = vec![
            Tile::Basic(BasicTile::new(TileColor::Red, 8)),
            Tile::Basic(BasicTile::new(TileColor::Blue, 8)),
            Tile::Joker(Joker::new(JokerVariant::Double)),
            Tile::Basic(BasicTile::new(TileColor::Black, 8)),
        ];
        assert_eq!(is_valid_set(set), false);
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
        assert_eq!(is_valid_set(set), true);
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
        assert_eq!(is_valid_set(set), true);
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
        assert_eq!(is_valid_set(set), false);
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
        assert_eq!(is_valid_set(set), false);
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
        assert_eq!(is_valid_set(set), false);
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
        assert_eq!(is_valid_set(set), false);
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
        assert_eq!(is_valid_set(set), true);
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
        assert_eq!(is_valid_set(set), false);
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
        assert_eq!(is_valid_set(set), false);
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
        assert_eq!(is_valid_set(set), true);
    }

    #[test]
    fn test_valid_run_with_two_color_change_sandwich() {
        let set = vec![
            Tile::Joker(Joker::new(JokerVariant::ColorChange)),
            Tile::Basic(BasicTile::new(TileColor::Red, 8)),
            Tile::Joker(Joker::new(JokerVariant::ColorChange)),
        ];
        assert_eq!(is_valid_set(set), true);
    }

    #[test]
    fn test_valid_run_with_two_color_change_adjacent() {
        let set = vec![
            Tile::Basic(BasicTile::new(TileColor::Orange, 6)),
            Tile::Joker(Joker::new(JokerVariant::ColorChange)),
            Tile::Joker(Joker::new(JokerVariant::ColorChange)),
        ];
        assert_eq!(is_valid_set(set), true);
    }

    #[test]
    fn test_adjacent_color_change_house_rule() {
        let set = vec![
            Tile::Basic(BasicTile::new(TileColor::Orange, 6)),
            Tile::Joker(Joker::new(JokerVariant::ColorChange)),
            Tile::Joker(Joker::new(JokerVariant::ColorChange)),
            Tile::Basic(BasicTile::new(TileColor::Blue, 9)),
        ];
        assert_eq!(is_valid_set(set), true);
    }

    #[test]
    fn test_adjacent_color_change_house_rule_2() {
        let set = vec![
            Tile::Basic(BasicTile::new(TileColor::Orange, 6)),
            Tile::Joker(Joker::new(JokerVariant::ColorChange)),
            Tile::Joker(Joker::new(JokerVariant::ColorChange)),
            Tile::Basic(BasicTile::new(TileColor::Orange, 9)),
        ];
        assert_eq!(is_valid_set(set), true);
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
        assert_eq!(is_valid_set(set), true);
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
        assert_eq!(is_valid_set(set), true);
    }
}
