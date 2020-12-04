/// Copyright (c) 2020, Shoyo Inokuchi
use crate::parser::is_valid_set;
use crate::tiles::Tile;

pub fn can_win(board: &Vec<Vec<Tile>>, rack: &Vec<Tile>) -> Result<(), ()> {
    for set in board {
        if !is_valid_set(set) {
            panic!("Initial board contains an invalid set: {:?}", set);
        }
    }
    Err(())
}
