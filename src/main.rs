use std::io;

enum Player {
    X,
    O,
}

fn main() {
    let mut playfield = [[' ', ' ', ' '], [' ', ' ', ' '], [' ', ' ', ' ']];
    let mut player = Player::X;
    let mut move_count = 0;

    'game: loop {
        match player {
            Player::X => println!("Spieler X ist dran"),
            Player::O => println!("Spieler O ist dran"),
        }

        let mut coordinatex = String::new();
        let mut coordinatey = String::new();

        println!("Enter your move:");
        println!("Enter x (0-2):");
        io::stdin()
            .read_line(&mut coordinatex)
            .expect("Fehler");

        // Parse as usize so the value can index the board without casts.
        let coordinatex: usize = match coordinatex.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Bitte eine Zahl eingeben (0-2)!");
                continue 'game;
            }
        };

        println!("Enter y (0-2):");
        io::stdin()
            .read_line(&mut coordinatey)
            .expect("Fehler");

        let coordinatey: usize = match coordinatey.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Bitte eine Zahl eingeben (0-2)!");
                continue 'game;
            }
        };

        if coordinatex > 2 || coordinatey > 2 {
            println!("Fehler");
            continue 'game;
        }

        if playfield[coordinatey][coordinatex] != ' ' {
            println!("Besetzt");
            continue 'game;
        }

        playfield[coordinatey][coordinatex] = match player {
            Player::X => 'X',
            Player::O => 'O',
        };

        for row in 0..3 {
            for col in 0..3 {
                if col != 2 {
                    print!("  {}  |", playfield[row][col]);
                } else {
                    print!("  {}  ", playfield[row][col]);
                }
            }

            println!();

            if row != 2 {
                println!("-----------------");
            }
        }

        let mark = match player {
            Player::X => 'X',
            Player::O => 'O',
        };

        // A win is three matching marks in a row, column, or diagonal.
        let row_win = (0..3).any(|row| {
            playfield[row][0] == mark
                && playfield[row][1] == mark
                && playfield[row][2] == mark
        });
        let col_win = (0..3).any(|col| {
            playfield[0][col] == mark
                && playfield[1][col] == mark
                && playfield[2][col] == mark
        });
        let main_diag_win =
            playfield[0][0] == mark && playfield[1][1] == mark && playfield[2][2] == mark;
        let anti_diag_win =
            playfield[0][2] == mark && playfield[1][1] == mark && playfield[2][0] == mark;

        if row_win || col_win || main_diag_win || anti_diag_win {
            match player {
                Player::X => println!("Spieler X hat gewonnen"),
                Player::O => println!("Spieler O hat gewonnen"),
            }
            break 'game;
        }

        // Draw is only possible after confirming that nobody won this turn.
        move_count += 1;
        if move_count == 9 {
            println!("Unentschieden");
            break 'game;
        }

        player = match player {
            Player::X => Player::O,
            Player::O => Player::X,
        };
    }
}
