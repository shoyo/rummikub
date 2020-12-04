/// Copyright (c) 2020, Shoyo Inokuchi
use rummikub::solve::can_win;
use rummikub::tiles::{BasicTile, Joker, JokerVariant, Tile, TileColor, TileValue};

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

    match can_win(&board, &rack) {
        Ok(moves) => {
            println!("{:?}", moves);
        }
        Err(_) => println!("No winning move."),
    }
}
