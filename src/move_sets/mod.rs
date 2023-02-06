pub mod pawn_moves;

const RAYS: [[u64; 64]; 8] = rays();

/*Directions are clockwise starting from north
0: north
1: northeast
2: east
3: southeast
4: south
5: southwest
6: west
7: northwest*/
const fn rays() -> [[u64; 64]; 8] {
    let mut res = [[0; 64]; 8];
    let mut i = 0;
    //Bad code but computed once :pensive: (no for loop in const)
    while i < 64 {
        let x: u64 = 1 << i;
        res[0][i] = north_one(x);
        res[1][i] = north_east_one(x);
        res[2][i] = east_one(x);
        res[3][i] = south_east_one(x);
        res[4][i] = south_one(x);
        res[5][i] = south_west_one(x);
        res[6][i] = west_one(x);
        res[7][i] = north_west_one(x);
        let mut j = 0;
        while j < 6 {
            res[0][i] |= north_one(res[0][i]);
            res[1][i] |= north_east_one(res[1][i]);
            res[2][i] |= east_one(res[2][i]);
            res[3][i] |= south_east_one(res[3][i]);
            res[4][i] |= south_one(res[4][i]);
            res[5][i] |= south_west_one(res[5][i]);
            res[6][i] |= west_one(res[6][i]);
            res[7][i] |= north_west_one(res[7][i]);
            j += 1;
        }
        i += 1;
    }
    res
}

pub const KING_MOVES: [u64; 64] = king_moves();

const fn king_moves() -> [u64; 64] {
    let mut res = [0; 64];
    let mut i = 0;
    while i < 64 {
        let x: u64 = 1 << i;
        res[i] = east_one(x)
            | west_one(x)
            | north_one(x)
            | north_east_one(x)
            | north_west_one(x)
            | south_one(x)
            | south_east_one(x)
            | south_west_one(x);
        i += 1;
    }
    res
}

pub const KNIGHT_MOVES: [u64; 64] = knight_moves();

const fn knight_moves() -> [u64; 64] {
    let mut res = [0; 64];
    let mut i = 0;
    //May be slow, idk tbh, but its precomupted lmao
    while i < 64 {
        let x: u64 = 1 << i;
        let mut y: u64 = south_east_one(south_one(x)) | south_west_one(south_one(x));
        y |= north_east_one(north_one(x)) | north_west_one(north_one(x));
        y |= south_west_one(west_one(x)) | north_west_one(west_one(x));
        y |= south_east_one(east_one(x)) | north_east_one(east_one(x));
        res[i] = y;
        i += 1;
    }
    res
}

const NOT_A_FILE: u64 = 0xfefefefefefefefe;
const NOT_H_FILE: u64 = 0x7f7f7f7f7f7f7f7f;

#[inline]
pub const fn east_one(set: u64) -> u64 {
    (set << 1) & NOT_A_FILE
}

#[inline]
pub const fn west_one(set: u64) -> u64 {
    (set >> 1) & NOT_H_FILE
}

#[inline]
pub const fn north_one(set: u64) -> u64 {
    set << 8
}

#[inline]
pub const fn north_east_one(set: u64) -> u64 {
    (set << 9) & NOT_A_FILE
}

#[inline]
pub const fn north_west_one(set: u64) -> u64 {
    (set << 7) & NOT_H_FILE
}

#[inline]
pub const fn south_one(set: u64) -> u64 {
    set >> 8
}

#[inline]
pub const fn south_east_one(set: u64) -> u64 {
    (set >> 7) & NOT_A_FILE
}

#[inline]
pub const fn south_west_one(set: u64) -> u64 {
    (set >> 9) & NOT_H_FILE
}

//Source: https://rhysre.net/fast-chess-move-generation-with-magic-bitboards.html
#[inline]
pub const fn bishop_moves(square: usize, empty: u64) -> u64 {
    let blockers = !empty;
    let mut attacks: u64 = 0;

    // North West
    attacks |= RAYS[7][square];
    if RAYS[7][square] & blockers != 0 {
        let blocker_index = u64::trailing_zeros(RAYS[7][square] & blockers) as usize;
        attacks &= !RAYS[7][blocker_index];
    }

    // North East
    attacks |= RAYS[1][square];
    if RAYS[1][square] & blockers != 0 {
        let blocker_index = u64::trailing_zeros(RAYS[1][square] & blockers) as usize;
        attacks &= !RAYS[1][blocker_index];
    }

    // South East
    attacks |= RAYS[3][square];
    if RAYS[3][square] & blockers != 0 {
        let blocker_index = u64::leading_zeros(RAYS[3][square] & blockers) as usize;
        attacks &= !RAYS[3][63 - blocker_index];
    }

    // South West
    attacks |= RAYS[5][square];
    if RAYS[5][square] & blockers != 0 {
        let blocker_index = u64::leading_zeros(RAYS[5][square] & blockers) as usize;
        attacks &= !RAYS[5][63 - blocker_index];
    }
    attacks
}

//Source: https://rhysre.net/fast-chess-move-generation-with-magic-bitboards.html
#[inline]
pub const fn rook_moves(square: usize, empty: u64) -> u64 {
    let blockers = !empty;
    let mut attacks: u64 = 0;

    // North
    attacks |= RAYS[0][square];
    if RAYS[0][square] & blockers != 0 {
        let blocker_index = u64::trailing_zeros(RAYS[0][square] & blockers) as usize;
        attacks &= !RAYS[0][blocker_index];
    }

    // East
    attacks |= RAYS[2][square];
    if RAYS[2][square] & blockers != 0 {
        let blocker_index = u64::trailing_zeros(RAYS[2][square] & blockers) as usize;
        attacks &= !RAYS[2][blocker_index];
    }

    // West
    attacks |= RAYS[6][square];
    if RAYS[6][square] & blockers != 0 {
        let blocker_index = u64::leading_zeros(RAYS[6][square] & blockers) as usize;
        attacks &= !RAYS[6][63 - blocker_index];
    }

    // South
    attacks |= RAYS[4][square];
    if RAYS[4][square] & blockers != 0 {
        let blocker_index = u64::leading_zeros(RAYS[4][square] & blockers) as usize;
        attacks &= !RAYS[4][63 - blocker_index];
    }

    attacks
}

#[inline]
pub fn queen_moves(square: usize, empty: u64) -> u64 {
    rook_moves(square, empty) | bishop_moves(square, empty)
}