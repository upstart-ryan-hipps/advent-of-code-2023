use regex::Regex;
use std::io;

fn main() {
    let mut sum = 0;
    let mut inputs = String::new();
    while inputs.trim() != "end" {
        inputs = "".to_string();
        io::stdin().read_line(&mut inputs).expect("failed to read line");
        sum += get_numbers(&mut inputs);
    }
    
    println!("{:?}", sum);
}

fn get_numbers(input: &mut String) -> i32 {
    let no_alpha = Regex::new(r"([^a-zA-Z])").unwrap();

    let mut numbers = vec![];
    for (_, [number]) in no_alpha.captures_iter(input).map(|c| c.extract()) {
        numbers.push(number);
    }        

    let mut number = String::new();
    if numbers.len() > 1 {
        number = numbers[0].to_owned() + numbers[numbers.len() -2];
        match number.parse::<i32>() {
            Ok(number) => { return number },
            Err(msg) => { return 0 }
        }
    }
    return 0;
}
