use std::collections::HashMap;

fn main() {
    let roman = "MCMXCIV";
    let integer = 1994;

    match roman_to_integer(roman) {
        Some(value) => println!("The Roman numeral {} equals {}", roman, value),
        None => println!("Invalid Roman numeral: {}", roman),
    }

    println!("The integer {} equals {}", integer, integer_to_roman(integer));
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
