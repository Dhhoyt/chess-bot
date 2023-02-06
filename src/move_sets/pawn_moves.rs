use crate::move_sets::*;

pub const WHITE_SINGLE_PUSH_TARGETS: [u64; 64] = white_single_push_targets();

const fn white_single_push_targets() -> [u64; 64] {
    let mut res = [0; 64];
    let mut i = 0;
    while i < 64 {
        let x: u64 = 1 << i;
        res[i] = north_one(x);
        i += 1;
    }
    res
}

pub const WHITE_DOUBLE_PUSH_TARGETS: [u64; 64] = white_double_push_targets();

const fn white_double_push_targets() -> [u64; 64] {
    let mut res = [0; 64];
    let mut i = 0;
    const RANK_4: u64 = 0x00000000FF000000;
    while i < 64 {
        let x: u64 = 1 << i;
        res[i] = north_one(north_one(x)) & RANK_4;
        i += 1;
    }
    res
}

pub const BLACK_SINGLE_PUSH_TARGETS: [u64; 64] = black_single_push_targets();

const fn black_single_push_targets() -> [u64; 64] {
    let mut res = [0; 64];
    let mut i = 0;
    while i < 64 {
        let x: u64 = 1 << i;
        res[i] = south_one(x);
        i += 1;
    }
    res
}

pub const BLACK_DOUBLE_PUSH_TARGETS: [u64; 64] = black_double_push_targets();

const fn black_double_push_targets() -> [u64; 64] {
    let mut res = [0; 64];
    let mut i = 0;
    const RANK_5: u64 = 0x000000FF00000000;
    while i < 64 {
        let x: u64 = 1 << i;
        res[i] = south_one(south_one(x)) & RANK_5;
        i += 1;
    }
    res
}

pub const PAWN_ATTACKS: [[u64; 64]; 2] = pawn_attacks();

const fn pawn_attacks() -> [[u64; 64]; 2] {
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