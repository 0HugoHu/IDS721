use std::io;

fn convert_to_roman(num: i32) -> String {
    let mut result = String::new();
    let mut n = num;

    let pairs = vec![
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
        (1, "I")
    ];

    for pair in pairs {
        while n >= pair.0 {
            result.push_str(pair.1);
            n -= pair.0;
        }
    }

    result
}

fn main() {
    println!("Enter a number to convert to Roman numerals:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num = input.trim().parse::<i32>().expect("Invalid input");

    let roman_num = convert_to_roman(num);
    println!("{} -> {}", num, roman_num);
}
