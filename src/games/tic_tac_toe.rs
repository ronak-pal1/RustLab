use std::{collections::HashMap, io};


// Terminal based TIC TAC TOE game
pub fn play() {

    let mut player1 = String::new();
    let mut player2 = String::new();


    println!("Enter name of player1");
    io::stdin().read_line(&mut player1).expect("Failed to read player1 name");

    println!("Enter name of player2");
    io::stdin().read_line(&mut player2).expect("Failed to read player2 name");
    

    println!("Starting the TIC TAC TOE game\n\n");


    let mut game_tbl: HashMap<i32, char> = HashMap::new();

    // 9 boxes.. so total 9 iterations
    for i in 0..9 {

        let mut input = String::new();

        let mut current_player = player1.clone();
        let mut symbol = 'x';


        if i%2 != 0 {
             current_player = player2.clone();
             symbol = 'o';
        }

        println!("\n\nEnter position for {:?} from 0 to 8", current_player);


        io::stdin().read_line(&mut input).expect("Failed to read input");


        let pos: i32 = input.trim().parse().expect("Enter a valid number");

        game_tbl.insert(pos, symbol);

        print_game_table(&game_tbl);

    }


    // Checking the winner


}



fn print_game_table(h:&HashMap<i32,char>) {

    for i in 0..9 {

        let value = h.get(&i);

        match value {
            Some(c) => {
                print!("{} ", *c);
            }
            None => {
                print!("_ ");
            }
        }

        if (i+1) % 3 == 0 {
            println!("");
        }

    }

}