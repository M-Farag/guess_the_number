use std::io;
use crate::game::guess;

pub mod game;
fn main() {
    let mut game_1 = guess::Game::start(
        String::from("Guess a number between 1 & 100"),
        0 );
    'main_app_loop:loop{
        let mut user_input:String = String::new();
        println!("{}",game_1.welcome_message);
        io::stdin().read_line(&mut user_input).expect("Err reading your number");
        
        game_1.user_input = match user_input.trim().parse() {
            Err(_) => continue,
            Ok(n) => n
        };

        game_1.compare();
        

    }
    

}
