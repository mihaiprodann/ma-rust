use std::io; 

struct Computer {
    brand: String,
    processor_name: String,
    memory_size: i32
}

fn new(brand: String, processor_name: String, memory_size: i32) -> Computer {
    Computer {
        brand: brand,
        processor_name: processor_name,
        memory_size: memory_size
    }
}

fn display(computer: Computer) {
    println!("Computer stats:\n--------");
    println!("Brand: {}", computer.brand);
    println!("Processor name: {}", computer.processor_name);
    println!("Memory size: {}\n", computer.memory_size);
}

fn read_i32() -> i32 {
    let line = io::stdin().lines().next().unwrap().unwrap();
    line.parse().unwrap()
}

fn main() {
    let computers = [
        new(String::from("Acer"), String::from("i7"), 16),
        new(String::from("Dell"), String::from("i5"), 8),
        new(String::from("HP"), String::from("i3"), 4)
    ];
    println!("Computers menu:");
    println!("1. Display all computers");
    println!("2. Print the computer with the largest memory\n");

    let input = read_i32();

    if input == 1 {
        for computer in computers {
            display(computer);
        }
    }
    else {
        let mut computer_with_max : Computer = new(String::from(""), String::from(""), 0);
        for computer in computers {
            if computer.memory_size > computer_with_max.memory_size {
                computer_with_max = computer;
            }
        }
        display(computer_with_max);
    }
}