/// Copyright (c) 2020, Shoyo Inokuchi

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
        JokerVariant::ColorChange => "COLOR_CHANGE".to_string(),
    }
}

enum Set {
    Run(Run),
    Group(Group),
}

struct Run {
    tiles: Vec<Tile>,
}

impl Run {
    fn new(tiles: Vec<Tile>) -> Result<Self, String> {
        if tiles.len() < 3 {
            return Err(format!("Attempted to create a run with fewer than 3 tiles"));
        }
        let color: TileColor;
        match tiles[0] {
            Tile::Basic => color = 
        }
        let color = tiles[0].color;
        for tile in tiles.iter() {}
        Ok(Self { tiles })
    }

    fn len(&self) -> u8 {
        self.end - self.start + 1
    }

    fn add_tile(&mut self, tile: Tile) -> Result<(), String> {
        if tile.color != self.color {
            return Err(format!(
                "Attempted to insert tile with color {} into run with color {}",
                format_color(tile.color),
                format_color(self.color)
            ));
        }
        if tile.value != self.start - 1 || tile.value != self.end + 1 {
            return Err(format!(
                "Attempted to insert tile with value {} into run with values {}~{}",
                tile.value, self.start, self.end
            ));
        }
        if tile.value == self.start - 1 {
            self.start -= 1;
        } else {
            self.end += 1;
        }
        Ok(())
    }

    /// Split the run at the boundary. Self is mutated into front half of the split, while the back
    /// half of the split is returned.
    ///
    /// EXAMPLE (split at boundary "6"):
    ///
    /// BEFORE:
    ///     Original 3 - 4 - 5 - 6 - 7 - 8
    ///
    /// AFTER:
    ///     Original 3 - 4 - 5
    ///     Returned 6 - 7 - 8
    fn split(&mut self, boundary: u8) -> Result<Run, String> {
        if self.len() < 6 {
            return Err(format!("Attempted to split a run with fewer than 6 tiles"));
        }
        if boundary < self.start
            || boundary - self.start < 3
            || self.end < boundary
            || self.end - boundary < 3
        {
            return Err(format!(
                "Attempted to split a run from {}~{} at an invalid boundary of {}",
                self.start, self.end, boundary
            ));
        }
        let run = Run::new(boundary, self.end, self.color).unwrap();
        self.end = boundary - 1;
        Ok(run)
    }

    /// Split the run and add a tile to the divide. Self is mutated into front half of the split,
    /// while the back half of the split is returned. The added tile becomes the head of the back
    /// half.
    ///
    /// EXAMPLE (split and add tile "6"):
    ///
    /// BEFORE:
    ///     Original 4 - 5 - 6 - 7 - 8
    ///
    /// AFTER:
    ///     Original 4 - 5 - 6
    ///     Returned 6 - 7 - 8
    fn split_and_add_tile(&mut self, tile: Tile) -> Result<Run, String> {
        if self.len() < 5 {
            return Err(format!(
                "Attempted to split and add tile to a run with fewer than 5 tiles"
            ));
        }
        if tile.value < self.start + 2 || tile.value > self.end - 2 {
            return Err(format!(
                "Attempted to split a run into insufficient lengths"
            ));
        }
        let run = Run::new(tile.value, self.end, self.color).unwrap();
        self.end = tile.value;
        Ok(run)
    }
}

struct Group {
    value: u8,
    colors: [bool; 4],
}

impl Group {
    fn new(value: u8, colors: [bool; 4]) -> Result<Self, ()> {
        Ok(Self { value, colors })
    }

    fn add_tile(&mut self, tile: Tile) -> Result<(), String> {
        Err(format!(""))
    }
}

fn can_win(board: Vec<Set>, rack: Vec<Tile>) -> Result<(), ()> {
    Err(())
}

fn main() {
    let board = vec![];
    let rack = vec![
        Tile::Basic(BasicTile::new(TileColor::Red, 5)),
        Tile::Basic(BasicTile::new(TileColor::Red, 6)),
        Tile::Basic(BasicTile::new(TileColor::Red, 7)),
    ];

    match can_win(board, rack) {
        Ok(moves) => {
            println!("{:?}", moves);
        }
        Err(_) => println!("No winning move."),
    }
}
