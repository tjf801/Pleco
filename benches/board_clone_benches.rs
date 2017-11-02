#![feature(test)]
extern crate pleco;
extern crate test;
extern crate rand;

#[macro_use]
extern crate lazy_static;

use pleco::{SQ,BitBoard,Player,Piece,Board};
use pleco::tools::*;

use test::{black_box, Bencher};

lazy_static! {
    pub static ref RAND_BOARDS: Vec<Board> = {
        let mut vec = Vec::new();
        vec.push(Board::default());
        for x in 0..10 { vec.push(gen_rand_legal_board()); }
        vec
    };
}


#[bench]
fn bench_board_10_clone(b: &mut Bencher) {
    b.iter(|| {
        for board in RAND_BOARDS.iter() {
            black_box(board.shallow_clone());
        }
    })
}