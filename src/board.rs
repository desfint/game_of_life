use rand::{distributions::Uniform, *};

pub struct Board {
    pub board: Vec<Vec<CellState>>,
    size: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CellState {
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

    fn calculate_cell(&self, (x, y): (usize, usize), cell: &CellState) -> CellState {
        let mut count = 0;

        for i in ((x as i32)-1)..=((x as i32)+1) {
            if i < 0 || i > (self.size-1) as i32 { continue }

            for j in ((y as i32)-1)..=((y as i32)+1) {
                if j < 0 || j > (self.size-1) as i32 { continue }

                match self.board[j as usize][i as usize] {
                    CellState::LiveCell => count += 1,
                    CellState::DeadCell => (),
                }
            }
        }

        if *cell == CellState::LiveCell {
            count -= 1;

            if count == 2 || count == 3 {
                CellState::LiveCell
            } else {
                CellState::DeadCell
            }
        } else {
            if count == 3 {
                CellState::LiveCell
            } else {
                CellState::DeadCell
            }
        }


    }

    pub fn calculate_board(&self) -> Self {
        let board = &self.board;

        let next: Vec<Vec<CellState>> = self.board.iter().enumerate().map(|(y, row)| {
            row.iter().enumerate().map(|(x, cell)| {
                self.calculate_cell((x, y), cell)
            }).collect()
        }).collect();


        Board { board: next, size: board.len() as u32 }
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

        write!(f, "{}", buff.trim_end())
    }
}
