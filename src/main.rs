use std::io::stdin;
use std::thread;
use std::time::Duration;

struct MagicSquare {
    board: Vec<Vec<usize>>,
    board_size: usize,
}

impl MagicSquare {
    pub fn new(n: usize) -> Self {
        Self {
            board_size: n,
            board: vec![vec![0; n]; n],
        }
    }

    fn print_board(&self) {
        thread::sleep(Duration::from_millis(800));
        println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
        for v in self.board.iter() {
            for vv in v.iter() {
                print!("{:>4} ", vv);
            }
            println!();
        }
        println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    }

    pub fn perform(&mut self) {
        let mid = self.board_size / 2;
        let board_len = self.board_size - 1;
        let max_n = self.board_size * self.board_size;

        let mut col = 0;
        let mut row = mid;
        let mut val = 1;

        self.print_board();

        while val <= max_n {
            let mut cell_has_val = self.board[col][row] != 0;

            if !cell_has_val {
                self.board[col][row] = val;
                self.print_board();
                val += 1;
            }

            let col_top = col == 0;
            let row_leftmost = row == 0;

            let col_copy = col;
            let row_copy = row;

            if col_top && !row_leftmost {
                col = board_len;
                row -= 1;
            } else if !row_leftmost {
                col -= 1;
                row -= 1;
            } else if row_leftmost && col_top {
                col = board_len;
                row = board_len;
            } else if row_leftmost && !col_top {
                col -= 1;
                row = board_len;
            }

            cell_has_val = self.board[col][row] != 0;

            if cell_has_val {
                col = col_copy + 1;
                row = row_copy;
            }
        }
    }
}

fn main() {
    let mut buf = String::new();
    let stdin = stdin();

    println!("Input an Odd Number: ");

    let _ = stdin.read_line(&mut buf).expect("Failed reading input");
    let inp = buf.trim().parse::<usize>();

    match inp {
        Ok(n) if n % 2 != 0 => {
            let mut magic_square = MagicSquare::new(n);
            magic_square.perform();
        }
        _ => println!("Invalid Input"),
    }
}
