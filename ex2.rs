fn main() {
    let number1 = 30;
    let number2 = 27;

    let bigger_number = if number1 > number2 { number1 } else { number2 };

    println!("The bigger number between {} and {} is {}\n", number1, number2, bigger_number);

}