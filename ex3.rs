fn is_divisible(number: i32, divisor: i32) -> bool {
    number % divisor == 0
}

fn main() {
    let number = 3;
    if is_divisible(number, 2) {
        println!("{} is divisible by 2\n", number);
    } else {
        println!("{} is not divisible by 2\n", number);
    }
}