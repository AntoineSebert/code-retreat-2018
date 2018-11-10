use std::io;

fn get_keyboard_input() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            println!("{} bytes read", n);
        }
        Err(error) => println!("error: {}", error),
    }
    return input;
}

fn main() {
    let mut player1_name = String::new();
    let mut player2_name = String::new();
    {
        println!("Enter player one name");
        player1_name = get_keyboard_input();
        println!("{}", player1_name);
    }
    {
        println!("Enter player two name");
        player2_name = get_keyboard_input();
        println!("{}", player1_name);
    }
     
}
