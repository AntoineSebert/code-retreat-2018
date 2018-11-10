use std::io;
use std::env;
use std::process;

/*
 * Holds the player data.
 */
struct Player {
    name: String,
    score: u8,
    games_won: u8,
}

/*
 * Get a trimmed String from the keyword.
 */
fn get_keyboard_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    return input.trim().to_string();
}

/*
 * Get a u8 from the keyboard.
 * Value can be 0.
 */
fn get_parsed_input() -> u8 {
    let input = get_keyboard_input();  
    match input.parse::<u8>() {
        Ok(n) => return n,
        Err(error) => {
            println!("error: {}", error);
            return 0;
        }, 
    }
}

/*
 * Get a valid player number, between 0 and PLAYER_NUMBER
 */
fn get_player_number(player_number: u8) -> u8 {
    loop {
        println!("Enter Player number, between 0 and {}", player_number);
        let number = get_parsed_input(); 
        if number < player_number {
            return number;
        }
    } 
}

/*
 * Returns an array of Player of size PLAYER_NUMBER, initialized with a user-defined name.
 * Does not check for the reuse of names nor empty names.
 */
fn initialize_players(player_number: u8) -> Vec<Player> {
    let mut players = Vec::new();
    let mut n = 0;
    while n < player_number {
        println!("Enter player name");
        players.push(Player {
            name: get_keyboard_input(),
            score: 0,
            games_won: 0,
        });
        println!("Player name is {}", players[players.len() - 1].name);
        n += 1;
    }
    return players;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // check arguments number
    if args.len() != 2 {
        println!("Incorrect number of arguments - correct syntax : main.exe number");
        process::exit(1);
    }

    // check the candidate argument for the player number
    let player_number: u8;
    match &args[1].parse::<u8>() {
        &Ok(n) => player_number = n,
        &Err(ref error) => {
            println!("error: {}", error);
            process::exit(1);
        },
    }
 
    let players: Vec<Player> = initialize_players(player_number);
 
    let mut count = 0u8;
    loop {
        println!("{}", count);
        count += 1;
        if count == 5 {
            break;
        }
    } 
}
