use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let result = if args.len() < 2 {
        "".to_string()
    } else {
        args[1..].join(" ")
    };
    println!("{}", result);
}
