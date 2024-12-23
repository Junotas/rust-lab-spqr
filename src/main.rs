use std::collections::HashMap;
use std::io;

#[derive(Debug)]
enum RomanError {
    InvalidCharacter(char),
    EmptyInput,
    OutOfRange(i32),
}

// Function to convert Roman numerals to integers, ignore this just for a github action test
fn roman_to_integer(input: &str) -> Result<i32, RomanError> {
    let roman = input.trim().to_uppercase();
    if roman.is_empty() {
        return Err(RomanError::EmptyInput);
    }

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

    for c in roman.chars().rev() {
        let value = match map.get(&c) {
            Some(&val) => val,
            None => return Err(RomanError::InvalidCharacter(c)),
        };
        if value < prev_value {
            total -= value;
        } else {
            total += value;
        }
        prev_value = value;
    }

    // Here is where we check the range to actually use OutOfRange:
    if total < 1 || total > 3999 {
        return Err(RomanError::OutOfRange(total));
    }

    Ok(total)
}

// Function to convert an integer to Roman numerals
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
    for &(val, symbol) in &values {
        while num >= val {
            result.push_str(symbol);
            num -= val;
        }
    }
    result
}

enum UserChoice {
    IntegerToRoman,
    RomanToInteger,
    Exit,
    Invalid,
}

fn get_user_choice() -> UserChoice {
    println!("\nChoose an option:");
    println!("1: Convert a year to Roman numerals");
    println!("2: Convert Roman numerals to a year");
    println!("3: Exit");

    let mut choice = String::new();
    if io::stdin().read_line(&mut choice).is_err() {
        return UserChoice::Invalid;
    }

    match choice.trim() {
        "1" => UserChoice::IntegerToRoman,
        "2" => UserChoice::RomanToInteger,
        "3" => UserChoice::Exit,
        _ => UserChoice::Invalid,
    }
}

// Function to handle conversion from integer to Roman numerals
fn handle_integer_to_roman() {
    println!("Enter a year (e.g., 44 for AD 44, -44 for 44 BC).");

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        println!("Failed to read input.");
        return;
    }

    let raw_year = match input.trim().parse::<i32>() {
        Ok(y) => y,
        Err(_) => {
            println!("Invalid input. Please enter a valid integer.");
            return;
        }
    };

    if raw_year == 0 {
        println!("There is no year 0 in standard BC/AD dating. Please try again.");
        return;
    }

    let abs_year = raw_year.abs();
    if abs_year < 1 || abs_year > 3999 {
        println!("Year out of range. Must be between 1 and 3999 (ignoring sign).");
        return;
    }

    let roman = integer_to_roman(abs_year);
    if raw_year < 0 {
        println!("{} BC is represented as \"{}\" in Roman numerals.", abs_year, roman);
    } else {
        println!("AD {} is represented as \"{}\" in Roman numerals.", raw_year, roman);
    }
}

// Function to handle conversion from Roman numerals to integer
fn handle_roman_to_integer() {
    println!("Enter Roman numerals (e.g., XIV, mmxix, etc.):");

    let mut roman = String::new();
    if io::stdin().read_line(&mut roman).is_err() {
        println!("Failed to read input.");
        return;
    }

    match roman_to_integer(&roman) {
        Ok(year) => {
            println!("Roman numeral \"{}\" is {}", roman.trim(), year);
        },
        Err(err) => match err {
            RomanError::EmptyInput => {
                println!("Input was empty. Please enter valid Roman numerals.")
            }
            RomanError::InvalidCharacter(c) => {
                println!("Invalid character '{}' found. Please use only I, V, X, L, C, D, or M.", c);
            }
            RomanError::OutOfRange(y) => {
                println!("Year {} is out of supported range (1-3999).", y);
            }
        }
    }
}

fn main() {
    loop {
        match get_user_choice() {
            UserChoice::IntegerToRoman => handle_integer_to_roman(),
            UserChoice::RomanToInteger => handle_roman_to_integer(),
            UserChoice::Exit => {
                println!("Exiting the program. Goodbye!");
                break;
            }
            UserChoice::Invalid => {
                println!("Invalid option. Please try again.");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests for converting integers to Roman numerals
    #[test]
    fn test_integer_to_roman_basic() {
        assert_eq!(integer_to_roman(1), "I");
        assert_eq!(integer_to_roman(4), "IV");
        assert_eq!(integer_to_roman(9), "IX");
        assert_eq!(integer_to_roman(40), "XL");
        assert_eq!(integer_to_roman(44), "XLIV");
        assert_eq!(integer_to_roman(3999), "MMMCMXCIX");
    }

    // Tests for converting Roman numerals to integers
    #[test]
    fn test_roman_to_integer_basic() {
        assert_eq!(roman_to_integer("I").unwrap(), 1);
        assert_eq!(roman_to_integer("IV").unwrap(), 4);
        assert_eq!(roman_to_integer("IX").unwrap(), 9);
        assert_eq!(roman_to_integer("XL").unwrap(), 40);
        assert_eq!(roman_to_integer("XLIV").unwrap(), 44);
        assert_eq!(roman_to_integer("MMMCMXCIX").unwrap(), 3999);
        assert_eq!(roman_to_integer("i").unwrap(), 1);
        assert_eq!(roman_to_integer("mmxix").unwrap(), 2019);
        assert!(matches!(roman_to_integer(""), Err(RomanError::EmptyInput)));
        assert!(matches!(roman_to_integer("ABCD"), Err(RomanError::InvalidCharacter(_))));
        assert!(matches!(roman_to_integer("MMMM"), Err(RomanError::OutOfRange(4000))));
    }
}
