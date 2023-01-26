use crate::{north_one, south_one};

fn white_single_push_targets(white_pawns: u64,  empty: u64) -> u64 {
    north_one(white_pawns) & empty
}

fn white_double_push_targets(white_pawns: u64, empty: u64) -> u64 {
    const RANK_4: u64 = 0x00000000FF000000;
    let single_pushes = white_single_push_targets(white_pawns, empty);
    north_one(single_pushes) & empty & RANK_4
}

fn black_single_push_targets(black_pawns: u64,  empty: u64) -> u64 {
    south_one(black_pawns) & empty
}

fn black_double_push_targets(black_pawns: u64, empty: u64) -> u64 {
    const RANK_5: u64 = 0x000000FF00000000;
    let single_pushes = black_single_push_targets(black_pawns, empty);
    south_one(single_pushes) & empty & RANK_5
}

const fn pawn_attacks() -> [[u64; 64]; 2] {
    let mut res = [[0; 64]; 2];
    let mut x: u64 = 1;
    while x <= 0x8000000000000000 {
        x <<= 1;
    }
    res
}

fn white_rams(white_pawns: u64, black_pawns: u64) -> u64 {
    south_one(black_pawns) & white_pawns
}

fn black_rams(white_pawns: u64, black_pawns: u64) -> u64 {
    north_one(white_pawns) & black_pawns
}