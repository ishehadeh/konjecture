use indexmap::IndexMap;
use konane::{bitboard::Direction, Konane256, TileState};
use std::{
    collections::{HashSet, VecDeque},
    fs,
    io::Read,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GameClass {
    Frac(i32, u32),
    Number(isize),
    Nimber(isize),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum GameAnalysisState {
    Waiting,
    Complete(GameClass),
}

#[derive(Debug, Default)]
pub struct GameAnalyzer {
    pub states: IndexMap<Konane256<16, 16>, GameAnalysisState>,
    pub work_queue: VecDeque<usize>,
}

impl GameAnalyzer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn submit_game(&mut self, game: Konane256<16, 16>) {
        if self.states.contains_key(&game) {
            return;
        }
        self.states.insert(game, GameAnalysisState::Waiting);
        self.work_queue.push_back(self.states.len() - 1);
    }

    pub fn analyze_next(&mut self) -> bool {
        let Some(idx) = self.work_queue.pop_front() else {
            return false;
        };

        let (game, state) = self
            .states
            .get_index(idx)
            .expect("indexes from work queue should always be valid");
        if state != &GameAnalysisState::Waiting {
            return true;
        }
        let game = game.clone();

        let mut white_moves: HashSet<GameClass> = Default::default();
        let mut black_moves: HashSet<GameClass> = Default::default();
        let mut needs_child = false;
        for dir in Direction::all() {
            let w_move_set = game.moveset(true, dir);
            for move_i in w_move_set.board.iter_set() {
                let mut moved = game.clone();
                moved.white.board.set(move_i);

                let mut clear_i = move_i as isize + 1 * dir.x() + 16 * dir.y();
                while !moved.white.board.get(clear_i as usize) {
                    moved.black.board.clear(clear_i as usize);
                    clear_i += 1 * dir.x() + 16 * dir.y();
                }
                moved.white.board.clear(clear_i as usize);

                if let Some(GameAnalysisState::Complete(c)) = self.states.get(&moved) {
                    white_moves.insert(*c);
                } else {
                    needs_child = true;
                    self.submit_game(moved);
                }
            }

            let b_move_set = game.moveset(false, dir);
            for move_i in b_move_set.board.iter_set() {
                let mut moved = game.clone();
                let mut clear_i = move_i as isize + 1 * dir.x() + 16 * dir.y();
                while !moved.black.board.get(clear_i as usize) {
                    moved.white.board.clear(clear_i as usize);
                    clear_i += 1 * dir.x() + 16 * dir.y();
                }
                moved.black.board.clear(clear_i as usize);

                if let Some(GameAnalysisState::Complete(c)) = self.states.get(&moved) {
                    black_moves.insert(*c);
                } else {
                    needs_child = true;
                    self.submit_game(moved);
                }
            }
        }

        if needs_child {
            self.work_queue.push_back(idx);
            return true;
        }

        let max_w = white_moves
            .iter()
            .map(|v| match v {
                GameClass::Number(n) => n,
                a => todo!("{a:?}"),
            })
            .min();
        let max_b = black_moves
            .iter()
            .map(|v| match v {
                GameClass::Number(n) => n,
                a => todo!("{a:?}"),
            })
            .max();
        let value = match (max_b, max_w) {
            (None, None) => 0,
            (None, Some(r)) => r - 1,
            (Some(l), None) => l + 1,
            (Some(0), Some(0)) => {
                println!("can't handle nimbers yet! using zero...");
                0
            }
            (Some(l), Some(r)) => l + r,
        };
        self.states
            .insert(game, GameAnalysisState::Complete(GameClass::Number(value)));
        true
    }
}

pub fn main() {
    let board_text = {
        let mut txt = String::new();
        fs::File::open("konane.txt")
            .expect("failed to open file")
            .read_to_string(&mut txt)
            .expect("failed to read file");
        txt
    };

    let mut game = Konane256::<16, 16>::empty();
    for (y, row_txt) in board_text.split("\n").take(16).enumerate() {
        for (x, c) in row_txt.chars().take(16).enumerate() {
            match c {
                'x' => game.set_tile(x, y, TileState::White),
                'o' => game.set_tile(x, y, TileState::Black),
                '.' => game.set_tile(x, y, TileState::Empty),
                _ => panic!("invalid tile character: {:?}", c),
            }
        }
    }

    let mut analyzer = GameAnalyzer::default();
    analyzer.submit_game(game);

    while analyzer.analyze_next() {}

    println!("{:?}", analyzer.states.get_index(0));
}
