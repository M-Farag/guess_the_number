use std::{io, process::exit};
use rand::Rng;
use std::cmp::Ordering;

struct Game {
    machine_code:usize,
    welcome_message:String,
    user_input:usize
}

impl Game {
    fn start(welcome_message:String,user_input:usize) -> Self
    {
        Self { 
            machine_code: rand::thread_rng().gen_range(1..=100), 
            welcome_message,
            user_input
         }
    }

    fn compare(&self) {
        match &self.user_input.cmp(&self.machine_code) {
            Ordering::Equal => {println!("You win");exit(1)},
            Ordering::Greater => println!("Too High, guess a bit low"),
            Ordering::Less => println!("Too small, guess a bit higher"),
        };
    }

}
fn main() {
    let mut game_1 = Game::start(
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
