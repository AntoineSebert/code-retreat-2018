use std::io;

struct Player {
    name: String,
    score: u8,
    games_won: u8,
}

fn get_keyboard_input() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => println!("{} bytes read", n),
        Err(error) => println!("error: {}", error),
    }
    return input;
}

fn main() {
    let mut player1 = Player {
        name: String::new(),
        score: 0,
        games_won: 0,
    };
    let mut player2 = Player {
        name: String::new(),
        score: 0,
        games_won: 0,
    };
    {
        println!("Enter player one name");
        player1.name = get_keyboard_input();
    }
    {
        println!("Enter player two name");
        player2.name = get_keyboard_input();
    }
    println!("PLayer one is {} and Player two is {}", player1.name, player2.name);
    
    let mut count = 0u8;
    loop {
        println!("{}", count);
        count += 1;
        if count == 5 {
            break;
        }
    }
}
