use std::io;

fn main() {
  println!("Please enter your name:");
  let mut name = String::new();
  io::stdin().read_line(&mut name).expect("Error reading input.");
  println!("Hello, {}!", name)
}
