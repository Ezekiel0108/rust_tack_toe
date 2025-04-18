use colored::*;
use rand::Rng;
use std::io;

//apparently we have : in front of a variable name in order to choose the type
fn main() {
    println!("{}", "This is a game of tick-tack-toe!".purple());

    let mut play_count = 0;
    let mut board: [[&str; 3]; 3] = [[""; 3]; 3];
    let mut player: &str = "-";

    initialize_board_matrix(&mut board);
    loop {
        print_board(board);
        player_insert(&mut play_count, &mut player, &mut board);
        //means if check_win == true
        if check_win(board) {
            //this is all just one println, useful to know for readability
            println!(
                "{}{}{}",
                "Congrats! ".yellow(),
                player.blue(),
                " has won!".yellow()
            );
            break;
        }
    }
}

// &mut is in the typization because and &mut is a type
// in which we pass the address and can modify it,wierd i know...
fn initialize_board_matrix(brd: &mut [[&str; 3]; 3]) {
    for x in 0..3 {
        for y in 0..3 {
            brd[x][y] = "-";
        }
    }
}

fn print_board(brd: [[&str; 3]; 3]) {
    println!("1 2 3");
    println!("{}│{}│{} 1", brd[0][0], brd[0][1], brd[0][2]);
    println!("─┼─┼─");
    println!("{}│{}│{} 2", brd[1][0], brd[1][1], brd[1][2]);
    println!("─┼─┼─");
    println!("{}│{}│{} 3", brd[2][0], brd[2][1], brd[2][2]);
}

fn check_win(brd: [[&str; 3]; 3]) -> bool {
    let mut win = false;

    //column win check
    for x in 0..3 {
        if (brd[x][0] == "x" && brd[x][1] == "x" && brd[x][2] == "x")
            || (brd[x][0] == "o" && brd[x][1] == "o" && brd[x][2] == "o")
        {
            win = true;
            return win;
        }
    }
    //row win check
    for y in 0..3 {
        if (brd[0][y] == "x" && brd[1][y] == "x" && brd[2][y] == "x")
            || (brd[0][y] == "o" && brd[1][y] == "o" && brd[2][y] == "o")
        {
            win = true;
            return win;
        }
    }
    //left to right slanted check
    if (brd[0][0] == "x" && brd[1][1] == "x" && brd[2][2] == "x")
        || (brd[0][0] == "o" && brd[1][1] == "o" && brd[2][2] == "o")
    {
        win = true;
        return win;
    }
    //right to left slanted check
    if (brd[2][0] == "x" && brd[1][1] == "x" && brd[0][2] == "x")
        || (brd[2][0] == "o" && brd[1][1] == "o" && brd[0][2] == "o")
    {
        win = true;
        return win;
    }
    win
}

fn actual_insert() -> usize {
    let mut p_insert = String::new();
    let mut ins: i32;

    loop {
        io::stdin()
            .read_line(&mut p_insert)
            .expect("Couldn't read line");
        ins = match p_insert.trim().parse() {
            Ok(num) => num,
            // _ is a catch all value, it is used to catch any error
            Err(_) => {
                println!("{}", "Only input numbers!".red());
                //this is a return required by Err
                4
            }
        };
        if ins < 1 || ins > 3 {
            println!("{}", "Only insert numbers going from 1 to 3!".red());
            println!(
                "{}",
                "Please, re-make your selection, going from 1 to 3".yellow()
            );
            //Bug of endless loop fixed by re initialiazing p_insert, i am a GOD.
            p_insert = String::new();
            continue;
        } else {
            break;
        }
    }
    ins as usize - 1
}

fn player_insert(playcnt: &mut i32, plyr: &mut &str, brd: &mut [[&str; 3]; 3]) {
    //r = row
    let mut r;
    //c = column
    let mut c;

    //* is used to de-reference, meaning saying to your compiler that we aren't changing the
    //address but just the contents inside of it, GODDAMN THIS TOOK A WHILE
    if *playcnt == 0 {
        println!("{}", "Randomly selecting who starts...".yellow());
        let rand = rand::rng().random_range(1..=2);
        if rand == 1 {
            *plyr = "x";
        } else {
            *plyr = "o";
        }
        println!("{}{}", plyr.green(), " begins".yellow());
    }
    //Still just one println, wander why the compiler turns it to this...
    println!(
        "{}{}{}",
        "Player ".white(),
        plyr.green(),
        " make your move!".purple()
    );

    loop {
        println!("{}", "Select a row you want to play in (1-3)".yellow());
        r = actual_insert();
        println!("{}", "Select a column you want to play in (1-3)".yellow());
        c = actual_insert();
        if brd[c][r] == "-" {
            break;
        } else {
            println!("{}", "The spot is already taken!".red());
            continue;
        }
    }

    if *plyr == "x" {
        brd[c][r] = "x";
        *plyr = "o";
    } else {
        brd[c][r] = "o";
        *plyr = "x";
    }

    *playcnt += 1;
}
