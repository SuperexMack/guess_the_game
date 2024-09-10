// guess game
use std::io;
use rand :: Rng;
use std::cmp::Ordering;

fn master(){

  loop {
    let secret_number = rand::thread_rng().gen_range(1..=10);

      // now we are going to take the input from the user

      println!("Welcome sir please guess a value :");
      let mut guessed_number  = String::new();
      io::stdin().read_line(&mut guessed_number).expect("you had not entered a numeber");

      let new_guessed_number:u32 = guessed_number.trim().parse().expect("Something went wrong");

      match new_guessed_number.cmp(&secret_number){
        Ordering::Equal => println!("You won"),
        Ordering::Greater => println!("Your loose"),
        Ordering::Less => println!("Your loose"),
    }
}
  
}

fn main(){
  master()
}