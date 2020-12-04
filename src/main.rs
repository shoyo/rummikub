/// Copyright (c) 2020, Shoyo Inokuchi
use rummikub::tiles::{BasicTile, Joker, JokerVariant, Tile, TileColor, TileValue};

fn can_win(_board: Vec<Vec<Tile>>, _rack: Vec<Tile>) -> Result<(), ()> {
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
