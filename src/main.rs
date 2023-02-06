//#![allow(dead_code)]

use move_sets::{pawn_moves::*, *};

mod move_sets;

fn main() {
    for i in perft(4) {
        println!("{}", chess_board_to_fen(i, false))
    }
}

fn perft(mut depth_plies: usize) -> Vec<ChessBoard>{
    depth_plies -= 1;
    let mut res = ChessBoard::default().psuedo_white_moves();
    let mut white: bool = false;
    while depth_plies > 0 {
        let mut temp = Vec::new();
        if white {
            for i in res {
                temp.extend(i.psuedo_white_moves());
            }
            white = false;
        } else {
            for i in res {
                temp.extend(i.psuedo_black_moves());
            }
            white = true;
        }
        res = temp;
        depth_plies -= 1;
    }
    res
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
    fifty_move: usize,
    move_number: usize
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
                castle: 0x0000000000000081,
            },
            black_pieces: BitBoard {
                kings: 0x1000000000000000,
                queens: 0x0800000000000000,
                rooks: 0x8100000000000000,
                bishops: 0x2400000000000000,
                knights: 0x4200000000000000,
                pawns: 0x00FF000000000000,
                en_pessant: 0,
                castle: 0x8100000000000000,
            },
            move_number: 1,
            fifty_move: 0
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
            for i in BitBoardIter(KING_MOVES[king] & !self.occupied_by_white()) {
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
            for i in BitBoardIter(KNIGHT_MOVES[knight] & !self.occupied_by_white()) {
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
                WHITE_DOUBLE_PUSH_TARGETS[i] & self.empty() & north_one(self.empty()),
            ) {
                let mut t = self.clone();
                t.white_pieces.pawns = (t.white_pieces.pawns + (1 << i)) & mask;
                t.black_pieces.en_pessant = 0;
                t.white_pieces.en_pessant = south_one(1 << i);
                res.push(t);
                t.fifty_move = 0;
            }
            for i in BitBoardIter(WHITE_SINGLE_PUSH_TARGETS[i] & self.empty()) {
                let mut t = self.clone();
                t.white_pieces.pawns = (t.white_pieces.pawns + (1 << i)) & mask;
                t.black_pieces.en_pessant = 0;
                res.push(t);
                t.fifty_move = 0;
            }
            for i in BitBoardIter(PAWN_ATTACKS[0][i] & self.occupied_by_black()) {
                let mut t = self.clone();
                t.white_pieces.pawns = (t.white_pieces.pawns + (1 << i)) & mask;
                t.black_pieces.en_pessant = 0;
                t.capture_black(i);
                res.push(t);
            }
            for i in BitBoardIter(PAWN_ATTACKS[0][i] & self.black_pieces.en_pessant) {
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
        if self.occupied_by_black() & !mask != 0 {
            self.fifty_move = 0;
            self.black_pieces.queens &= mask;
            self.black_pieces.rooks &= mask;
            self.black_pieces.bishops &= mask;
            self.black_pieces.knights &= mask;
            self.black_pieces.pawns &= mask;
        } else {
            self.fifty_move += 1;
        }
    }

    fn psuedo_black_moves(&self) -> Vec<ChessBoard> {
        let mut res: Vec<ChessBoard> = Vec::new();
        for king in BitBoardIter(self.black_pieces.kings) {
            let mask: u64 = !(1 << king);
            for i in BitBoardIter(KING_MOVES[king] & !self.occupied_by_black()) {
                let mut t = self.clone();
                t.black_pieces.kings = (t.black_pieces.kings + (1 << i)) & mask;
                t.white_pieces.en_pessant = 0;
                t.capture_white(i);
                t.move_number += 1;
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
                t.move_number += 1;
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
                t.move_number += 1;
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
                t.move_number += 1;
                res.push(t);
            }
        }
        for knight in BitBoardIter(self.black_pieces.knights) {
            let mask: u64 = !(1 << knight);
            for i in BitBoardIter(KNIGHT_MOVES[knight] & !self.occupied_by_black()) {
                let mut t = self.clone();
                t.black_pieces.knights = (t.black_pieces.knights + (1 << i)) & mask;
                t.white_pieces.en_pessant = 0;
                t.capture_white(i);
                t.move_number += 1;
                res.push(t);
            }
        }
        for i in BitBoardIter(self.black_pieces.pawns) {
            let mask: u64 = !(1 << i);
            for i in BitBoardIter(
                BLACK_DOUBLE_PUSH_TARGETS[i] & self.empty() & south_one(self.empty()),
            ) {
                let mut t = self.clone();
                t.black_pieces.pawns = (t.black_pieces.pawns + (1 << i)) & mask;
                t.white_pieces.en_pessant = 0;
                t.black_pieces.en_pessant = north_one(1 << i);
                t.move_number += 1;
                t.fifty_move = 0;
                res.push(t);
            }
            for i in BitBoardIter(BLACK_SINGLE_PUSH_TARGETS[i] & self.empty()) {
                let mut t = self.clone();
                t.black_pieces.pawns = (t.black_pieces.pawns + (1 << i)) & mask;
                t.white_pieces.en_pessant = 0;
                t.move_number += 1;
                t.fifty_move = 0;
                res.push(t);
            }
            for i in BitBoardIter(PAWN_ATTACKS[1][i] & self.occupied_by_white()) {
                let mut t = self.clone();
                t.black_pieces.pawns = (t.black_pieces.pawns + (1 << i)) & mask;
                t.white_pieces.en_pessant = 0;
                t.capture_white(i);
                t.move_number += 1;
                res.push(t);
            }
            for i in BitBoardIter(PAWN_ATTACKS[1][i] & self.white_pieces.en_pessant) {
                let mut t = self.clone();
                t.black_pieces.pawns = (t.black_pieces.pawns + (1 << i)) & mask;
                t.white_pieces.en_pessant = 0;
                t.capture_white(i + 8);
                t.move_number += 1;
                res.push(t);
            }   
        }
        res.into_iter().filter(|b| (b.under_attack_by_white() & self.black_pieces.kings) == 0).collect()
    }

    #[inline]
    fn capture_white(&mut self, square: usize) {
        let mask = !(1 << square);
        if self.occupied_by_white() & !mask != 0{
            self.fifty_move = 0;
            self.white_pieces.queens &= mask;
            self.white_pieces.rooks &= mask;
            self.white_pieces.bishops &= mask;
            self.white_pieces.knights &= mask;
            self.white_pieces.pawns &= mask;
        } else {
            self.fifty_move += 1;
        }
    }

    #[inline]
    fn under_attack_by_white(&self) -> u64 {
        let mut res: u64 = 0;
        for i in BitBoardIter(self.white_pieces.kings) {
            res |= KING_MOVES[i];
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
            res |= KNIGHT_MOVES[i];
        }   
        for i in BitBoardIter(self.white_pieces.pawns) {
            res |= PAWN_ATTACKS[0][i];
        }
        res
    }
}

fn chess_board_to_fen(board: ChessBoard, white_move: bool) -> String {
    let mut filled = !board.empty();
    let mut res = String::new();
    let mut last_id = 0;
    for y in 0..8 {
        let mut blank_spaces = 0;
        for x in 0..8 {
            let square = (7 - y) * 8 + x;
            let mask: u64 = 1 << square;
            if filled & mask != 0 {
                if blank_spaces != 0 {
                    res.push_str(&blank_spaces.to_string());
                    blank_spaces = 0;
                }
                if board.black_pieces.kings & mask != 0 {
                    res.push('k');
                }
                else if board.black_pieces.queens & mask != 0 {
                    res.push('q');
                }
                else if board.black_pieces.rooks & mask != 0 {
                    res.push('r');
                }
                else if board.black_pieces.knights & mask != 0 {
                    res.push('n');
                }
                else if board.black_pieces.bishops & mask != 0 {
                    res.push('b');
                }
                else if board.black_pieces.pawns & mask != 0 {
                    res.push('p');
                }
                else if board.white_pieces.kings & mask != 0 {
                    res.push('K');
                }
                else if board.white_pieces.queens & mask != 0 {
                    res.push('Q');
                }
                else if board.white_pieces.rooks & mask != 0 {
                    res.push('R');
                }
                else if board.white_pieces.knights & mask != 0 {
                    res.push('N');
                }
                else if board.white_pieces.bishops & mask != 0 {
                    res.push('B');
                }
                else if board.white_pieces.pawns & mask != 0 {
                    res.push('P');
                }
            } else {
                blank_spaces += 1;
            }
        }
        if blank_spaces != 0 {
            res.push_str(&blank_spaces.to_string());
        }
        if y != 7 {
            res.push_str("/");
        }
    }
    res.push(' ');
    if white_move {
        res.push('w');
    } else {
        res.push('b');
    }
    res.push(' ');
    if board.white_pieces.castle & 0x0000000000000080 != 0 {
        res.push('K');
    }
    if board.white_pieces.castle & 0x0000000000000001 != 0 {
        res.push('Q');
    }
    if board.black_pieces.castle & 0x8000000000000000 != 0 {
        res.push('k');
    }
    if board.black_pieces.castle & 0x0100000000000000 != 0 {
        res.push('q');
    }
    res.push(' ');
    if (board.white_pieces.en_pessant | board.black_pieces.en_pessant) == 0 {
        res.push('-');
    } else {
        for i in BitBoardIter(board.white_pieces.en_pessant | board.black_pieces.en_pessant) {
            let rank = i/8 + 1;
            let file = i % 8;
            res.push(match file {
                0 => 'a',
                1 => 'b',
                2 => 'c',
                3 => 'd',
                4 => 'e',
                5 => 'f',
                6 => 'g',
                7 => 'h',
                _ => panic!()
            });
            res.push_str(&rank.to_string());
        }
    }
    res.push(' ');
    res.push_str(&board.fifty_move.to_string());
    res.push(' ');
    res.push_str(&board.move_number.to_string());
    res
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