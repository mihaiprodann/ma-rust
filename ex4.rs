fn main() {
    let a = [1, 5, 2, 3, 14, 8, 9, 7, 6, 10];
    let mut max = a[0];
    for element in a {
        if max < element {
            max = element;
        }
    }
    println!("The maximum from array is: {}\n", max);

}