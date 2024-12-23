use std::collections::HashMap;
use std::io;

fn main() {
    loop {
        println!("\nChoose an option:");
        println!("1: Convert a year to Roman numerals");
        println!("2: Convert Roman numerals to a year");
        println!("3: Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");

        match choice.trim() {
            "1" => {
                println!("Enter a year (1-3999):");
                let mut year = String::new();
                io::stdin().read_line(&mut year).expect("Failed to read input");

                if let Ok(year) = year.trim().parse::<i32>() {
                    if year > 0 && year <= 3999 {
                        println!("The Roman numeral for {} is {}", year, integer_to_roman(year));
                    } else {
                        println!("Year must be between 1 and 3999.");
                    }
                } else {
                    println!("Invalid input. Please enter a valid year.");
                }
            }
            "2" => {
                println!("Enter Roman numerals:");
                let mut roman = String::new();
                io::stdin().read_line(&mut roman).expect("Failed to read input");

                match roman_to_integer(roman.trim()) {
                    Some(value) => println!("The year for Roman numeral {} is {}", roman.trim(), value),
                    None => println!("Invalid Roman numeral input."),
                }
            }
            "3" => {
                println!("Exiting the program. Goodbye!");
                break;
            }
            _ => println!("Invalid option. Please try again."),
        }
    }
}

fn roman_to_integer(s: &str) -> Option<i32> {
    let mut map = HashMap::new();
    map.insert('I', 1);
    map.insert('V', 5);
    map.insert('X', 10);
    map.insert('L', 50);
    map.insert('C', 100);
    map.insert('D', 500);
    map.insert('M', 1000);

    let mut total = 0;
    let mut prev_value = 0;

    for c in s.chars().rev() {
        let value = map.get(&c)?;
        if *value < prev_value {
            total -= value;
        } else {
            total += value;
        }
        prev_value = *value;
    }

    Some(total)
}

fn integer_to_roman(mut num: i32) -> String {
    let values = [
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];

    let mut result = String::new();
    for &(value, symbol) in &values {
        while num >= value {
            result.push_str(symbol);
            num -= value;
        }
    }

    result
}
