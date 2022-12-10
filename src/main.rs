use std::io;

fn main() {
    println!("\nEnter the nth fibonacci position to get its value");
    println!("The fibonacci squence starts from 0\n");

    // getting the nth position from the user
    let position: u32 = loop {
        let mut position = String::new();

        io::stdin()
            .read_line(&mut position)
            .expect("Can't read your input");

        break match position.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Pleave input a positive number");
                continue;
            }
        };
    };

    let value: u64 = nth_fibonacci_value(position);
    println!(
        "\nThe fibonacci value at position {} is: {}\n",
        position, value
    )
}

fn nth_fibonacci_value(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => nth_fibonacci_value(n - 1) + nth_fibonacci_value(n - 2),
    }
}
