use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    let contents = fs::read_to_string("input1.txt").unwrap();
    let scratchies: Vec<String> = contents.lines().map(String::from).collect();
    let mut scores: Vec<i32> = vec![];
    let mut scratchy_scores = HashMap::new();
    let mut number_of_scratchies: i32 = scratchies.len().try_into().unwrap();

    for (idx, scratchy) in scratchies.iter().enumerate() {
        let mut itr = scratchy.split(":").take(2);
        let scratchy_id = itr.next().expect("gameid").to_string();
        let mut my_numbers_winning_numbers = itr.next().expect("numbers").split("|");
        let my_numbers: Vec<&str> = my_numbers_winning_numbers.next().expect("my numbers").split(" ").collect();
        let winning_numbers: Vec<&str> = my_numbers_winning_numbers.next().expect("my numbers").split(" ").collect();
        
        let my_numbers_set: HashSet<&str> = HashSet::from_iter(my_numbers.iter().cloned());
        let winning_numbers_set: HashSet<&str> = HashSet::from_iter(winning_numbers.iter().cloned());
        let intersection: HashSet<_> = my_numbers_set.intersection(&winning_numbers_set).collect();
        let power: u32 = (intersection.len()-1).try_into().unwrap();
        let score: i32 = if power > 0 { i32::pow(2, power-1) } else { 0 };
        //println!("Intersections: {}, {}, {:?}", score, intersection.len(), intersection);
        scores.push(score);

        let index: i32 = idx.try_into().unwrap();
        scratchy_scores.entry(index).or_insert(power as i32);
    }

    println!("\n Score of scratchy: {}",scores.iter().sum::<i32>());
    println!("\n Number of Scratchies: {}",scratchy_scores.keys().len());

    for scratchy_game_id in scratchy_scores.keys() {
        number_of_scratchies += get_number_of_copys_won(scratchy_scores.clone(), *scratchy_game_id);
    }

    println!("\n Total Scratchies: {}", number_of_scratchies);
}

fn get_number_of_copys_won(scratchies: HashMap<i32, i32>, current_game_id: i32) -> i32 { 
    if current_game_id as usize > scratchies.keys().len() { return 0 };
    let mut copies_won: i32 = *scratchies.get(&current_game_id).unwrap();
    //println!("Current ID: {}, copies won {}", current_game_id, copies_won);

    for copy in 1..=copies_won {
        //println!("Current ID: {}, checking_copy {}", current_game_id, copy);
        copies_won += get_number_of_copys_won(scratchies.clone(), current_game_id+copy);
    }

    return copies_won as i32;
}
