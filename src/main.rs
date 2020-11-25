/// Copyright (c) 2020, Shoyo Inokuchi
use std::collections::HashMap;

/// A single Rummikub tile
enum Tile {
    Basic(BasicTile),
    Joker(Joker),
}

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

#[derive(Eq, PartialEq)]
enum TileColor {
    Black,
    Red,
    Blue,
    Orange,
}

type TileValue = u8;

fn format_color(color_code: TileColor) -> String {
    match color_code {
        TileColor::Black => "BLACK".to_string(),
        TileColor::Red => "RED".to_string(),
        TileColor::Blue => "BLUE".to_string(),
        TileColor::Orange => "ORANGE".to_string(),
    }
}

struct Joker {
    variant: JokerVariant,
}

impl Joker {
    fn new(variant: JokerVariant) -> Self {
        Self { variant }
    }
}

enum JokerVariant {
    Single,
    Double,
    Mirror,
    ColorChange,
}

fn format_joker(variant: JokerVariant) -> String {
    match variant {
        JokerVariant::Single => "SINGLE".to_string(),
        JokerVariant::Double => "DOUBLE".to_string(),
        JokerVariant::Mirror => "MIRROR".to_string(),
        JokerVariant::ColorChange => "COLOR CHANGE".to_string(),
    }
}

enum Parsing {
    Run {
        last_value: TileValue,
        color: TileColor,
    },
    Group {
        seen: HashMap<TileColor, bool>,
    },
    Undetermined,
}

/// Given an ordered sequence of Rummikub tiles, return whether the sequence is valid.
fn is_valid_set(set: Vec<Tile>) -> bool {
    if set.len() < 3 {
        return false;
    }
    let mut parsing = Parsing::Undetermined;

    let mut tiles = set.iter().enumerate();
    while let Some((index, tile)) = tiles.next() {
        match parsing {
            Parsing::Run {
                ref last_value,
                ref color,
            } => match tile {
                Tile::Basic(t) => {
                    if t.value == 0 || t.value > 13 {
                        panic!(
                            "Illegal tile value {}: tiles should be 0 < value <= 13",
                            t.value
                        );
                    }
                    if t.value != last_value + 1 {
                        return false;
                    }
                    if t.color != *color {
                        return false;
                    }
                }
                Tile::Joker(j) => {}
            },
            Parsing::Group { ref mut seen } => {}
            Parsing::Undetermined => {}
        }
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
            Tile::Joker(Joker::new(JokerVariant::Mirror)),
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
            Tile::Joker(Joker::new(JokerVariant::Mirror)),
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
            Tile::Joker(Joker::new(JokerVariant::Mirror)),
            Tile::Basic(BasicTile::new(TileColor::Red, 8)),
            Tile::Basic(BasicTile::new(TileColor::Red, 9)),
        ];
        assert_eq!(is_valid_set(set), false);
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
