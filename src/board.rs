use rand::{distributions::Uniform, *};

pub struct Board {
    board: Vec<Vec<CellState>>,
    size: u32,
}

enum CellState {
    DeadCell,
    LiveCell,
}

impl Board {
    pub fn new(size: u32) -> Self {
        let range = Uniform::from(0..=1);
        let board = (0..size)
            .map(|_| {
                rand::thread_rng()
                    .sample_iter(&range)
                    .take(size as usize)
                    .map(|cell| if cell == 1 {CellState::LiveCell} else {CellState::DeadCell})
                    .collect()
            })
            .collect();

        Board { board, size }
    }


}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buff = String::new();
        self.board.iter().for_each(|row| {
            row.iter().for_each(|n| {
                match n {
                    CellState::LiveCell => buff.push_str("[]"),
                    CellState::DeadCell => buff.push_str("  "),
                }
            });
            buff.push('\n');
        });

        write!(f, "{}", buff)
    }
}
