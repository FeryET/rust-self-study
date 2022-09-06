use std::io;

fn main() {
    let mut buffer: String = String::new();
    io::stdin().read_line(&mut buffer).expect("Error occured in the program.");
    println!("X is {buffer}");
}
