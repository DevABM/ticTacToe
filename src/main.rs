use std::io::{self, Write};

struct Board {
    cells: [char; 9],
}

impl Board {
    fn new() -> Board {
        Board {
            cells: [' '; 9],
        }
    }

    fn print(&self) {
        println!("  {} | {} | {}", self.cells[0], self.cells[1], self.cells[2]);
        println!(" -----------");
        println!("  {} | {} | {}", self.cells[3], self.cells[4], self.cells[5]);
        println!(" -----------");
        println!("  {} | {} | {}", self.cells[6], self.cells[7], self.cells[8]);
    }

    fn is_full(&self) -> bool {
        for cell in self.cells.iter() {
            if *cell == ' ' {
                return false;
            }
        }
        true
    }

    fn make_move(&mut self, position: usize, symbol: char) -> Result<(), &'static str> {
        if position < 1 || position > 9 {
            return Err("Invalid move: position must be between 1 and 9.");
        }

        let index = position - 1;
        if self.cells[index] != ' ' {
            return Err("Invalid move: position already taken.");
        }

        self.cells[index] = symbol;
        Ok(())
    }

    fn has_winner(&self) -> bool {
        let lines = [
            // Rows
            [0, 1, 2], [3, 4, 5], [6, 7, 8],
            // Columns
            [0, 3, 6], [1, 4, 7], [2, 5, 8],
            // Diagonals
            [0, 4, 8], [2, 4, 6],
        ];

        for line in lines.iter() {
            if self.cells[line[0]] != ' ' &&
               self.cells[line[0]] == self.cells[line[1]] &&
               self.cells[line[1]] == self.cells[line[2]]
            {
                return true;
            }
        }

        false
    }
}

fn main() {
    let mut board = Board::new();
    let mut current_player = 'X';

    loop {
        board.print();

        println!("Player {}, enter your move (1-9):", current_player);

        let mut input = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let position: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number between 1 and 9.");
                continue;
            }
        };

        match board.make_move(position, current_player) {
            Ok(_) => {
                if board.has_winner() {
                    println!("Player {} wins!", current_player);
                    break;
                } else if board.is_full() {
                    println!("It's a tie!");
                    break;
                }

                current_player = if current_player == 'X' { 'O' } else { 'X' };
            }
            Err(err) => println!("{}", err),
        }
    }
}
