use crate::move_sets::*;

pub const fn white_single_push_targets(white_pawns: u64, empty: u64) -> u64 {
    north_one(white_pawns) & empty
}

pub const fn white_double_push_targets(white_pawns: u64, empty: u64) -> u64 {
    const RANK_4: u64 = 0x00000000FF000000;
    let single_pushes = white_single_push_targets(white_pawns, empty);
    north_one(single_pushes) & empty & RANK_4
}

pub const fn black_single_push_targets(black_pawns: u64, empty: u64) -> u64 {
    south_one(black_pawns) & empty
}

pub const fn black_double_push_targets(black_pawns: u64, empty: u64) -> u64 {
    const RANK_5: u64 = 0x000000FF00000000;
    let single_pushes = black_single_push_targets(black_pawns, empty);
    south_one(single_pushes) & empty & RANK_5
}

pub const fn pawn_attacks() -> [[u64; 64]; 2] {
    let mut res = [[0; 64]; 2];
    let mut i = 0;
    while i < 64 {
        let x: u64 = 1 << i;
        res[0][i] = north_west_one(x) | north_east_one(x);
        i += 1;
    }
    i = 0;
    while i < 64 {
        let x: u64 = 1 << i;
        res[1][i] = south_west_one(x) | south_east_one(x);
        i += 1;
    }
    res
}
