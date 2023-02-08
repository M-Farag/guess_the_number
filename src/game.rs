pub mod guess {
    use rand::Rng;
    use std::cmp::Ordering;
    use std::process::exit;

    pub struct Game {
        pub machine_code:usize,
        pub welcome_message:String,
        pub user_input:usize
    }
    
     impl Game {
        pub fn start(welcome_message:String,user_input:usize) -> Self
        {
            Self { 
                machine_code: rand::thread_rng().gen_range(1..=100), 
                welcome_message,
                user_input
             }
        }
    
        pub fn compare(&self) {
            match &self.user_input.cmp(&self.machine_code) {
                Ordering::Equal => {println!("You win");exit(1)},
                Ordering::Greater => println!("Too High, guess a bit low"),
                Ordering::Less => println!("Too small, guess a bit higher"),
            };
        }
    
    }
}