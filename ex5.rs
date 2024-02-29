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

fn main() {
    let computer = new(String::from("Acer"), String::from("i7"), 16);
    display(computer);
}