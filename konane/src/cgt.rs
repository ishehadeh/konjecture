use cgt::short::partizan::partizan_game::PartizanGame;

use crate::{bitboard::Direction, Konane256};

impl<const W: usize, const H: usize> PartizanGame for Konane256<W, H> {
    fn left_moves(&self) -> Vec<Self> {
        let mut moves: Vec<Self> = Default::default();
        for dir in Direction::all() {
            let moveset = self.moveset(false, dir);
            for move_i in moveset.board.iter_set() {
                let mut moved = self.clone();
                moved.black.board.set(move_i);

                let mut clear_i = move_i as isize + 1 * dir.x() + 16 * dir.y();
                while !moved.black.board.get(clear_i as usize) {
                    moved.white.board.clear(clear_i as usize);
                    clear_i += 1 * dir.x() + 16 * dir.y();
                }
                moved.black.board.clear(clear_i as usize);
                moves.push(moved);
            }
        }

        moves
    }

    fn right_moves(&self) -> Vec<Self> {
        let mut moves: Vec<Self> = Default::default();
        for dir in Direction::all() {
            let moveset = self.moveset(true, dir);
            for move_i in moveset.board.iter_set() {
                let mut moved = self.clone();
                moved.white.board.set(move_i);

                let mut clear_i = move_i as isize + 1 * dir.x() + 16 * dir.y();
                while !moved.white.board.get(clear_i as usize) {
                    moved.black.board.clear(clear_i as usize);
                    clear_i += 1 * dir.x() + 16 * dir.y();
                }
                moved.white.board.clear(clear_i as usize);
                moves.push(moved);
            }
        }

        moves
    }
}
