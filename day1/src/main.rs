use regex::Regex;
use std::io;

fn main() {
    let mut sum = 0u64;
    let mut numbers = vec![];
    let mut inputs = String::new();
    while inputs.trim() != "end" {
        inputs = "".to_string();
        io::stdin().read_line(&mut inputs).expect("failed to read line");
        numbers.push(get_numbers(&mut inputs));
    }

    for i in numbers {
        print!("\nnumber {}",i);
        sum += i
    }

    print!("\nsum {}",sum);
}

fn get_numbers(input: &mut String) -> u64 {
    
    let cap_numbers = Regex::new(r"([^a-zA-Z])|(one)|(two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine)").unwrap();

    let pre_process = pre_process_overlaps(input);
    let mut numbers = vec![];
    for (_, [number]) in cap_numbers.captures_iter(pre_process.as_str()).map(|c| c.extract()) {
        numbers.push(number);
    }        

    if numbers.len() > 1 {
        let first_num = text_to_number_text(numbers[0].to_owned());
        let last_num = text_to_number_text(numbers[numbers.len() -2].to_owned());
        let number = first_num + last_num.as_str();
        match number.parse::<u64>() {
            Ok(number) => { return number; },
            Err(_msg) => { return 0 }
        }
    }
    return 0;
}

fn text_to_number_text(input: String) -> String { 
    match input.trim() {
        "one"=>"1".to_owned(),
        "two"=>"2".to_owned(),
        "three"=>"3".to_owned(),
        "four"=>"4".to_owned(),
        "five"=>"5".to_owned(),
        "six"=>"6".to_owned(),
        "seven"=>"7".to_owned(),
        "eight"=>"8".to_owned(),
        "nine"=>"9".to_owned(),
        _=>input.to_owned(),
    }
}

fn pre_process_overlaps(input: &mut String) -> String {
    let fix_overlap_one = Regex::new(r"(twone)").unwrap(); 
    let fix_overlap_two = Regex::new(r"(eightwo)").unwrap();
    let fix_overlap_three = Regex::new(r"(eighthree)").unwrap();
    let fix_overlap_four = Regex::new(r"(sevenine)").unwrap();
    let fix_overlap_five = Regex::new(r"(threeight)").unwrap();
    let fix_overlap_six = Regex::new(r"(fiveight)").unwrap();
    let fix_overlap_seven = Regex::new(r"(nineight)").unwrap();
    let fix_overlap_eight = Regex::new(r"(oneight)").unwrap();
    let mut pre_process = fix_overlap_one.replace_all(input, "twoone").into_owned();
    pre_process = fix_overlap_two.replace_all(&pre_process, "eighttwo").into_owned();
    pre_process = fix_overlap_three.replace_all(&pre_process, "eightthree").into_owned();
    pre_process = fix_overlap_four.replace_all(&pre_process, "sevennine").into_owned();
    pre_process = fix_overlap_five.replace_all(&pre_process, "threeeight").into_owned();
    pre_process = fix_overlap_six.replace_all(&pre_process, "fiveeight").into_owned();
    pre_process = fix_overlap_seven.replace_all(&pre_process, "nineeight").into_owned();
    return fix_overlap_eight.replace_all(&pre_process, "oneeight").into_owned();
}
