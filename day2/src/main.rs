use std::fs;
use std::io;
use regex::Regex;
use std::collections::HashMap;

fn main() {

    let contents = fs::read_to_string("input1.txt").unwrap();

    print!("Reading input file...\n");
    io::Write::flush(&mut io::stdout()).expect("flush failed!");

    let games: Vec<String> = contents.lines().map(String::from).collect();
    let mut game_max_colors = HashMap::new();
    
    print!("Processing input file...\n");
    io::Write::flush(&mut io::stdout()).expect("flush failed!");

    // Process each game
    for game in games {
        // Split Game ID from game information
        let mut itr = game.split(":").take(2);
        let game_id_str = get_game_id(&itr.next().expect("gameid").to_string());
        let mut cube_draw_itr = itr.next().expect("cube_draw").split(";");

        // Process Game information
        let mut max_green = 0;
        let mut max_red = 0;
        let mut max_blue = 0;
        let mut max_color_hash = HashMap::new();
        for cube_draw in cube_draw_itr {
            let hash = get_cube_draw(cube_draw);
            max_green = get_max(max_green, *hash.get("green").unwrap_or(&0));
            max_red = get_max(max_red, *hash.get("red").unwrap_or(&0));
            max_blue = get_max(max_blue, *hash.get("blue").unwrap_or(&0));
        }
        
        max_color_hash.entry("green").or_insert(max_green);
        max_color_hash.entry("red").or_insert(max_red);
        max_color_hash.entry("blue").or_insert(max_blue);

        // Save the game maxes
        game_max_colors.entry(parse_num_str(&game_id_str)).or_insert(max_color_hash);
    }

    // Capture user input for total possible cubes
    let mut pos_green_cube = String::new();
    let mut pos_red_cube = String::new();
    let mut pos_blue_cube = String::new();

    ask_for_cube("green".to_string());
    io::stdin().read_line(&mut pos_green_cube).expect("failed to green line");
    ask_for_cube("red".to_string());
    io::stdin().read_line(&mut pos_red_cube).expect("failed to red line");
    ask_for_cube("blue".to_string());
    io::stdin().read_line(&mut pos_blue_cube).expect("failed to blue line");

    let mut green_cube = 0;
    match pos_green_cube.trim().parse::<i32>() {
        Ok(cube) => { green_cube = cube },
        Err(_msg) => { print!("\nno green") }
    }
    let mut red_cube = 0;
    match pos_red_cube.trim().parse::<i32>() {
        Ok(cube) => { red_cube = cube },
        Err(_msg) => { print!("\nno red") }
    }
    let mut blue_cube = 0;
    match pos_blue_cube.trim().parse::<i32>() {
        Ok(cube) => { blue_cube = cube },
        Err(_msg) => { print!("\nno blue") }
    }

    answer_the_elf(&mut game_max_colors, green_cube, red_cube, blue_cube);
}

fn answer_the_elf(game_board: &mut HashMap<i32, HashMap<&str, i32>>, green_cubes: i32, red_cubes: i32, blue_cubes: i32) {
    let mut game_ids: Vec<i32> = vec![];
    let mut powers: Vec<i32> = vec![];
    for (id, &mut ref thing) in game_board.iter_mut() {
        let green = *thing.get("green").unwrap_or(&0);
        let red = *thing.get("red").unwrap_or(&0);
        let blue = *thing.get("blue").unwrap_or(&0);
        print!("\n Actuals Green: {} Red: {} Blue: {}", green, red, blue);

        // Get possible games
        if green  <= green_cubes && red <= red_cubes && blue <= blue_cubes {
            game_ids.push(*id);
        }
        
        // Get game power
        powers.push(green*red*blue);
    }

    print!("{:?}", game_ids);
    print!("\n\n The Final Value is... {}", game_ids.iter().sum::<i32>());
    print!("\n\n The Sum of the Power of the min cubes is... {}",powers.iter().sum::<i32>());
}

fn get_game_id(line: &String) -> String {
    let game_capture = Regex::new(r"(\d+)").unwrap(); 
    return game_capture.captures(line).unwrap()[0].to_string();
}

fn parse_num_str(number: &String) -> i32 {
    match number.parse::<i32>() {
        Ok(number) => { return number; },
        Err(_msg) => { return 0 }
    }
}

fn get_cube_draw(cube_draw: &str) -> HashMap<&str, i32> {
    let mut result = HashMap::new();
    let cube_draw_capture = Regex::new(r"(\d+) (?<color>\w+)").unwrap();
    for (_, [count, color]) in cube_draw_capture.captures_iter(cube_draw).map(|c| c.extract()) {
        match count.parse::<i32>() {
            Ok(count) => { result.entry(color).or_insert(count); },
            Err(_msg) => { result.entry(color).or_insert(0); }
        }
        
    }

    return result;
}

fn get_max(item1: i32, item2: i32) -> i32 {
    if item1 < item2 { return item2 } else { return item1 }
}

fn ask_for_cube(color: String) {
    print!("Enter your number of {} cubes - ", color);
    io::Write::flush(&mut io::stdout()).expect("flush failed!");
}