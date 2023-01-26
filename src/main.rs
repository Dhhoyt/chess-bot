mod pawn_moves;

fn main() {
    println!("Hello, world!");
}

trait Bot<T: Board> {
    
}   

trait Board {
    fn white_moves(&self) -> Vec<Box<dyn Board>>;
    fn black_moves(&self) -> Vec<Box<dyn Board>>;
    fn simple_grade(&self) -> f32;
}

struct ChessBot {

}

impl Bot<ChessBoard> for ChessBot {
    
}

struct ChessBoard {
    white_pieces: BitBoard,
    black_pieces: BitBoard,
}

impl ChessBoard {
    fn empty(&self) -> u64{
        !(self.white_pieces.kings | self.white_pieces.queens | self.white_pieces.rooks | self.white_pieces.bishops | self.white_pieces.knights | self.white_pieces.pawns |
        self.black_pieces.kings | self.black_pieces.queens | self.black_pieces.rooks | self.black_pieces.bishops | self.black_pieces.knights | self.black_pieces.pawns)
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

    fn simple_grade(&self) -> f32 {
        let mut total: f32 = 0.;
        total += (self.white_pieces.queens.count_ones() - self.black_pieces.queens.count_ones()) as f32 * 9.;
        total += (self.white_pieces.rooks.count_ones() - self.black_pieces.rooks.count_ones()) as f32 * 5.;
        total += (self.white_pieces.bishops.count_ones() - self.black_pieces.bishops.count_ones()) as f32 * 3.;
        total += (self.white_pieces.knights.count_ones() - self.black_pieces.knights.count_ones()) as f32 * 3.;
        total += (self.white_pieces.pawns.count_ones() - self.black_pieces.pawns.count_ones()) as f32 * 1.;
        total
    }

    fn white_moves(&self) -> Vec<Box<dyn Board>> {
        Vec::new()
    }

    fn black_moves(&self) -> Vec<Box<dyn Board>> {
        Vec::new()
    }
}

fn north_one(set: u64) -> u64 {
    set << 8
}

fn south_one(set: u64) -> u64 {
    set >> 8
}