#![allow(dead_code)]
mod move_sets;

fn main() {
    let ser = searilize_bitboard(0xF0F);
    println!("{:?}", ser);
}

trait Bot<T: Board> {}

trait Board {
    fn white_moves(&self) -> Vec<Box<dyn Board>>;
    fn black_moves(&self) -> Vec<Box<dyn Board>>;
    fn hueristic(&self) -> f32;
}

struct ChessBot {}

impl Bot<ChessBoard> for ChessBot {}

struct ChessBoard {
    white_pieces: BitBoard,
    black_pieces: BitBoard,
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
}

struct BitBoard {
    kings: u64,
    queens: u64,
    rooks: u64,
    bishops: u64,
    knights: u64,
    pawns: u64,
}

impl Board for ChessBoard {
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

    fn white_moves(&self) -> Vec<Box<dyn Board>> {
        Vec::new()
    }

    fn black_moves(&self) -> Vec<Box<dyn Board>> {
        Vec::new()
    }
}

#[inline]
fn searilize_bitboard(mut board: u64) -> Vec<u32> {
    let mut res: Vec<u32> = Vec::new();
    while board != 0 {
        res.push(board.trailing_zeros());
        board &= !1 << board.trailing_zeros();
    }
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
}
