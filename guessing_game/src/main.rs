/* #[test]

fn it_accepts_a_guess() {
  guess(12);

  assert_eq!(get_guess(), 12)
}

#[test]

fn it_sets_a_guess() {
  guess(10);

  assert_eq!(get_guess(), 10)
}

fn main() {

}

struct Game {
  guess: i32
}

fn guess(a: i32) {

}

fn get_guess() -> i32 {
  12
} */

extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

  let something = 5 % 3;

  println!("Does modulo exist: {}", something);

  println!("Guess a number:");

  let secret_number = rand::thread_rng().gen_range(1, 101);

  loop {
    println!("Please input your guess");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .ok()
        .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
      };

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
      Ordering::Less    => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal   => {
        println!("You win!");
        break;
      }
    }
  }
}
