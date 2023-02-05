#![allow(dead_code)]

use move_sets::{pawn_moves::*, *};

mod move_sets;

fn main() {
    println!("{}", ChessBoard::default().psuedo_black_moves().len());
    let mut joe = ChessBoard::default().psuedo_white_moves();
    let mut jax: Vec<ChessBoard> = Vec::new();
    for i in joe.iter() {
        for j in i.psuedo_black_moves() {
            jax.push(j);
        }
    }
    println!("{}", jax.len());
    joe.clear();
    for i in jax.iter() {
        for j in i.psuedo_white_moves() {
            joe.push(j);
        }
    }
    jax.clear();
    for i in joe.iter() {
        for j in i.psuedo_black_moves() {
            jax.push(j);
        }
    }
    println!("{}", jax.len());
}

trait Bot<T: Board> {}

trait Board {
    fn white_moves(&self) -> Vec<Box<dyn Board>>;
    fn black_moves(&self) -> Vec<Box<dyn Board>>;
    fn hueristic(&self) -> f32;
}

struct ChessBot {}

impl ChessBot {}

#[derive(Clone, Copy)]
struct ChessBoard {
    white_pieces: BitBoard,
    black_pieces: BitBoard,
}

struct BitBoardIter(u64);

impl Iterator for BitBoardIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }
        let res = self.0.trailing_zeros() as usize;
        self.0 &= !(1 << self.0.trailing_zeros());
        Some(res)
    }
    
}

impl Default for ChessBoard {
    fn default() -> Self {
        ChessBoard {
            white_pieces: BitBoard {
                kings: 0x0000000000000010,
                queens: 0x0000000000000008,
                rooks: 0x0000000000000081,
                bishops: 0x0000000000000024,
                knights: 0x0000000000000042,
                pawns: 0x000000000000FF00,
                en_pessant: 0,
                castle: 0x0000000000000091,
            },
            black_pieces: BitBoard {
                kings: 0x1000000000000000,
                queens: 0x0800000000000000,
                rooks: 0x8100000000000000,
                bishops: 0x2400000000000000,
                knights: 0x4200000000000000,
                pawns: 0x00FF000000000000,
                en_pessant: 0,
                castle: 0x9100000000000000,
            },
        }
    }
}

impl ChessBoard {
    fn empty(&self) -> u64 {
        !(self.white_pieces.kings
            | self.white_pieces.queens
            | self.white_pieces.rooks
            | self.white_pieces.bishops
            | self.white_pieces.knights
            | self.white_pieces.pawns
            | self.black_pieces.kings
            | self.black_pieces.queens
            | self.black_pieces.rooks
            | self.black_pieces.bishops
            | self.black_pieces.knights
            | self.black_pieces.pawns)
    }

    fn occupied_by_white(&self) -> u64 {
        self.white_pieces.kings
            | self.white_pieces.queens
            | self.white_pieces.rooks
            | self.white_pieces.bishops
            | self.white_pieces.knights
            | self.white_pieces.pawns
    }

    fn occupied_by_black(&self) -> u64 {
        self.black_pieces.kings
            | self.black_pieces.queens
            | self.black_pieces.rooks
            | self.black_pieces.bishops
            | self.black_pieces.knights
            | self.black_pieces.pawns
    }
}

#[derive(Clone, Copy)]
struct BitBoard {
    kings: u64,
    queens: u64,
    rooks: u64,
    bishops: u64,
    knights: u64,
    pawns: u64,
    en_pessant: u64,
    castle: u64,
}

impl ChessBoard {
    fn hueristic(&self) -> f32 {
        let mut total: f32 = 0.;
        total += (self.white_pieces.queens.count_ones() - self.black_pieces.queens.count_ones())
            as f32
            * 9.;
        total += (self.white_pieces.rooks.count_ones() - self.black_pieces.rooks.count_ones())
            as f32
            * 5.;
        total += (self.white_pieces.bishops.count_ones() - self.black_pieces.bishops.count_ones())
            as f32
            * 3.;
        total += (self.white_pieces.knights.count_ones() - self.black_pieces.knights.count_ones())
            as f32
            * 3.;
        total += (self.white_pieces.pawns.count_ones() - self.black_pieces.pawns.count_ones())
            as f32
            * 1.;
        total
    }

    fn psuedo_white_moves(&self) -> Vec<ChessBoard> {
        let mut res: Vec<ChessBoard> = Vec::new();
        for king in BitBoardIter(self.white_pieces.kings) {
            let mask: u64 = !(1 << king);
            for i in BitBoardIter(king_moves()[king] & !self.occupied_by_white()) {
                let mut t = self.clone();
                t.white_pieces.kings = (t.white_pieces.kings + (1 << i)) & mask;
                t.black_pieces.en_pessant = 0;
                t.capture_black(i);
                res.push(t);
            }
        }
        for queen in BitBoardIter(self.white_pieces.queens) {
            let mask: u64 = !(1 << queen);
            for i in
                BitBoardIter(queen_moves(queen, self.empty()) & !self.occupied_by_white())
            {
                
                let mut t = self.clone();
                t.white_pieces.queens = (t.white_pieces.queens + (1 << i)) & mask;
                t.black_pieces.en_pessant = 0;
                t.capture_black(i);
                res.push(t);
            }
        }
        for rook in BitBoardIter(self.white_pieces.rooks) {
            let mask: u64 = !(1 << rook);
            for i in BitBoardIter(rook_moves(rook, self.empty()) & !self.occupied_by_white())
            {
                let mut t = self.clone();
                t.white_pieces.rooks = (t.white_pieces.rooks + (1 << i)) & mask;
                t.black_pieces.en_pessant = 0;
                t.capture_black(i);
                res.push(t);
            }
        }
        for bishop in BitBoardIter(self.white_pieces.bishops) {
            let mask: u64 = !(1 << bishop);
            for i in
                BitBoardIter(bishop_moves(bishop, self.empty()) & !self.occupied_by_white())
            {
                let mut t = self.clone();
                t.white_pieces.bishops = (t.white_pieces.bishops + (1 << i)) & mask;
                t.black_pieces.en_pessant = 0;
                t.capture_black(i);
                res.push(t);
            }
        }
        for knight in BitBoardIter(self.white_pieces.knights) {
            let mask: u64 = !(1 << knight);
            for i in BitBoardIter(knight_moves()[knight] & !self.occupied_by_white()) {
                let mut t = self.clone();
                t.white_pieces.knights = (t.white_pieces.knights + (1 << i)) & mask;
                t.black_pieces.en_pessant = 0;
                t.capture_black(i);
                res.push(t);
            }
        }
        for i in BitBoardIter(self.white_pieces.pawns) {
            let mask: u64 = !(1 << i);
            for i in BitBoardIter(
                white_double_push_targets()[i] & self.empty() & north_one(self.empty()),
            ) {
                let mut t = self.clone();
                t.white_pieces.pawns = (t.white_pieces.pawns + (1 << i)) & mask;
                t.black_pieces.en_pessant = 0;
                t.white_pieces.en_pessant = south_one(1 << i);
                res.push(t);
            }
            for i in BitBoardIter(white_single_push_targets()[i] & self.empty()) {
                let mut t = self.clone();
                t.white_pieces.pawns = (t.white_pieces.pawns + (1 << i)) & mask;
                t.black_pieces.en_pessant = 0;
                res.push(t);
            }
            for i in BitBoardIter(pawn_attacks()[0][i] & self.occupied_by_black()) {
                let mut t = self.clone();
                t.white_pieces.pawns = (t.white_pieces.pawns + (1 << i)) & mask;
                t.black_pieces.en_pessant = 0;
                t.capture_black(i);
                res.push(t);
            }
            for i in BitBoardIter(pawn_attacks()[0][i] & self.black_pieces.en_pessant) {
                print_bit_board(self.black_pieces.en_pessant);
                let mut t = self.clone();
                t.white_pieces.pawns = (t.white_pieces.pawns + (1 << i)) & mask;
                t.black_pieces.en_pessant = 0;
                t.capture_black(i - 8);
                res.push(t);
            }
        }
        res
    }

    fn capture_black(&mut self, square: usize) {
        let mask = !(1 << square);
        self.black_pieces.queens &= mask;
        self.black_pieces.rooks &= mask;
        self.black_pieces.bishops &= mask;
        self.black_pieces.knights &= mask;
        self.black_pieces.pawns &= mask;
    }

    fn psuedo_black_moves(&self) -> Vec<ChessBoard> {
        let mut res: Vec<ChessBoard> = Vec::new();
        for king in BitBoardIter(self.black_pieces.kings) {
            let mask: u64 = !(1 << king);
            for i in BitBoardIter(king_moves()[king] & !self.occupied_by_black()) {
                let mut t = self.clone();
                t.black_pieces.kings = (t.black_pieces.kings + (1 << i)) & mask;
                t.white_pieces.en_pessant = 0;
                t.capture_white(i);
                res.push(t);
            }
        }
        for queen in BitBoardIter(self.black_pieces.queens) {
            let mask: u64 = !(1 << queen);
            for i in
                BitBoardIter(queen_moves(queen, self.empty()) & !self.occupied_by_black())
            {
                let mut t = self.clone();
                t.black_pieces.queens = (t.black_pieces.queens + (1 << i)) & mask;
                t.white_pieces.en_pessant = 0;
                t.capture_white(i);
                res.push(t);
            }
        }
        for rook in BitBoardIter(self.black_pieces.rooks) {
            let mask: u64 = !(1 << rook);
            for i in BitBoardIter(rook_moves(rook, self.empty()) & !self.occupied_by_black())
            {
                let mut t = self.clone();
                t.black_pieces.rooks = (t.black_pieces.rooks + (1 << i)) & mask;
                t.white_pieces.en_pessant = 0;
                t.capture_white(i);
                res.push(t);
            }
        }
        for bishop in BitBoardIter(self.black_pieces.bishops) {
            let mask: u64 = !(1 << bishop);
            for i in
                BitBoardIter(bishop_moves(bishop, self.empty()) & !self.occupied_by_black())
            {
                let mut t = self.clone();
                t.black_pieces.bishops = (t.black_pieces.bishops + (1 << i)) & mask;
                t.white_pieces.en_pessant = 0;
                t.capture_white(i);
                res.push(t);
            }
        }
        for knight in BitBoardIter(self.black_pieces.knights) {
            let mask: u64 = !(1 << knight);
            for i in BitBoardIter(knight_moves()[knight] & !self.occupied_by_black()) {
                let mut t = self.clone();
                t.black_pieces.knights = (t.black_pieces.knights + (1 << i)) & mask;
                t.white_pieces.en_pessant = 0;
                t.capture_white(i);
                res.push(t);
            }
        }
        for i in BitBoardIter(self.black_pieces.pawns) {
            let mask: u64 = !(1 << i);
            for i in BitBoardIter(
                black_double_push_targets()[i] & self.empty() & south_one(self.empty()),
            ) {
                let mut t = self.clone();
                t.black_pieces.pawns = (t.black_pieces.pawns + (1 << i)) & mask;
                t.white_pieces.en_pessant = 0;
                t.black_pieces.en_pessant = north_one(1 << i);
                res.push(t);
            }
            for i in BitBoardIter(black_single_push_targets()[i] & self.empty()) {
                let mut t = self.clone();
                t.black_pieces.pawns = (t.black_pieces.pawns + (1 << i)) & mask;
                t.white_pieces.en_pessant = 0;
                res.push(t);
            }
            for i in BitBoardIter(pawn_attacks()[1][i] & self.occupied_by_white()) {
                let mut t = self.clone();
                t.black_pieces.pawns = (t.black_pieces.pawns + (1 << i)) & mask;
                t.white_pieces.en_pessant = 0;
                t.capture_white(i);
                res.push(t);
            }
            for i in BitBoardIter(pawn_attacks()[1][i] & self.white_pieces.en_pessant) {
                let mut t = self.clone();
                t.black_pieces.pawns = (t.black_pieces.pawns + (1 << i)) & mask;
                t.white_pieces.en_pessant = 0;
                t.capture_white(i + 8);
                res.push(t);
            }   
        }
        res.into_iter().filter(|b| (b.under_attack_by_white() & self.black_pieces.kings) == 0).collect()
    }

    fn capture_white(&mut self, square: usize) {
        let mask = !(1 << square);
        self.white_pieces.queens &= mask;
        self.white_pieces.rooks &= mask;
        self.white_pieces.bishops &= mask;
        self.white_pieces.knights &= mask;
        self.white_pieces.pawns &= mask;
    }

    #[inline]
    fn under_attack_by_white(&self) -> u64 {
        let mut res: u64 = 0;
        for i in BitBoardIter(self.white_pieces.kings) {
            res |= king_moves()[i];
        }
        for i in BitBoardIter(self.white_pieces.queens) {
            res |= queen_moves(i, self.empty());
        }
        for i in BitBoardIter(self.white_pieces.rooks) {
            res |= rook_moves(i, self.empty());
        }
        for i in BitBoardIter(self.white_pieces.bishops) {
            res |= bishop_moves(i, self.empty());
        }
        for i in BitBoardIter(self.white_pieces.knights) {
            res |= knight_moves()[i];
        }   
        for i in BitBoardIter(self.white_pieces.pawns) {
            res |= pawn_attacks()[0][i];
        }
        res
    }
}

fn print_bit_board(board: u64) {
    let mut bytes = board.to_ne_bytes();
    bytes.reverse();
    for i in bytes {
        let s = format!("{:#010b}", i);
        let s: String = s[2..s.len()].chars().rev().collect();
        println!("{}", s.replace("0", "."));
    }
    println!();
}