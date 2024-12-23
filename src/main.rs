use std::collections::HashMap;

fn main() {
    let roman = "MCMXCIV";
    match roman_to_integer(roman) {
        Some(value) => println!("The Roman numeral {} equals {}", roman, value),
        None => println!("Invalid Roman numeral: {}", roman),
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
