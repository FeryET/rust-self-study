fn next_fibonnaci(current: i32, previous: i32) -> i32 {
    current + previous
}
fn main() {
    let (mut current, mut previous) = (1, 1);
    let mut next;
    let mut sum = 0;
    while current < 4_000_000 {
        next = next_fibonnaci(current, previous);
        previous = current;
        current = next;
        if current % 2 == 0{
            sum = sum + current;
        };
    }
    println!("Summation is: {}", sum);
}
