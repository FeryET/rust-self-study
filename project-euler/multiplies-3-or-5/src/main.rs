fn is_multiply(number: i32, multiply: i32) -> bool{
    number % multiply == 0
}

fn main() {
    let mut sum = 0;
    for i in 0..1000 {
        if is_multiply(i, 3) || is_multiply(i, 5) {
            sum = sum + i;
        }
    }
    println!("Summation is {}", sum)
}
