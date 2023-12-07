use std::fs;

fn main() {
    let contents = fs::read_to_string("input1.txt").unwrap();
    let part_schemas: Vec<String> = contents.lines().map(String::from).collect();
    
    //Get Matrix dimensions
    let matrix_width = part_schemas[0].len();
    let matrix_height = part_schemas.len();
    let mut sum: Vec<i32> = Vec::new();
    let mut gear_math: Vec<i32> = Vec::new();
    print!("Matrix Width: {}, Matrix Height: {}", matrix_width, matrix_height);

    for (current_line_index, current_line) in part_schemas.iter().enumerate() {
        let mut cur_pos = 0;
        let prev_line = if current_line_index > 0 { &part_schemas[current_line_index - 1] } else { "" };
        let next_line = if current_line_index < matrix_height -1 { &part_schemas[current_line_index + 1] } else { "" };

        while cur_pos < matrix_width {
            // Check the neighbors of the current position
            let number_potential_gear: (bool, bool) = is_digit_with_adjacent_symbol(cur_pos, current_line.to_string(), prev_line.to_string(), next_line.to_string());

            // If the current position is a digit with a symbol neighbor go left and right and build the full digit
            let tupe: (i32, usize) = if number_potential_gear.0 { build_number(cur_pos, current_line.to_string()) } else { (0, cur_pos) };
            //println!("{:?}", tupe);
            sum.push(tupe.0);

            //If this is a gear find all surrounding numbers and if there are two multiply them together and add
            let surrounding_numbers = find_surrounding_gear_numbers(cur_pos, current_line.to_string(), prev_line.to_string(), next_line.to_string());
            if surrounding_numbers.len() == 2 { gear_math.push(surrounding_numbers[0] * surrounding_numbers[1]) }

            cur_pos = tupe.1 + 1;
        }
    }

    print!("\nTotal Sum: {}", sum.iter().sum::<i32>());
    print!("\nGears Sum {}", gear_math.iter().sum::<i32>());
}

// Return true to theres a possible part number, and return if it is a gear symbol
fn is_digit_with_adjacent_symbol(cur_pos: usize, cur_line: String, prev_line: String, next_line: String) -> (bool, bool) {
    let prev_char_array: Vec<char> = prev_line.chars().collect::<Vec<char>>().try_into().unwrap();
    let cur_char_array: Vec<char> = cur_line.chars().collect::<Vec<char>>().try_into().unwrap();
    let next_char_array: Vec<char> = next_line.chars().collect::<Vec<char>>().try_into().unwrap();

    let is_digit = cur_char_array[cur_pos].is_digit(10);
    if !is_digit { return (false, is_gear_symbol(cur_char_array[cur_pos])) }

    let check_prev = if prev_line.is_empty() { false } else { true };
    let check_next = if next_line.is_empty() { false } else { true };
    let check_left = if cur_pos <= 0 { false } else { true };
    let check_right = if cur_pos >= cur_line.len()-1 { false } else { true };

    // 1 | 4 | 6
    // 2 | c | 7
    // 3 | 5 | 8

    let left_column = (
        if check_prev && check_left { prev_char_array[cur_pos-1]} else { '.' }, 
        if check_left { cur_char_array[cur_pos-1] } else { '.' }, 
        if check_next && check_left { next_char_array[cur_pos-1] } else { '.' }
    );
    let center_column = (
        if check_prev { prev_char_array[cur_pos] } else { '.' }, 
        if check_next { next_char_array[cur_pos] } else { '.' }
    );
    let right_column = (
        if check_prev && check_right { prev_char_array[cur_pos+1] } else { '.' }, 
        if check_right { cur_char_array[cur_pos+1] } else { '.' }, 
        if check_next && check_right { next_char_array[cur_pos+1] } else { '.' }
    );

    // let undiscovered_gear = [is_gear_symbol(left_column.0), is_gear_symbol(center_column.0), is_gear_symbol(right_column.0),
    //                      is_gear_symbol(left_column.1), is_gear_symbol(center_column.1), is_gear_symbol(right_column.1),
    //                      is_gear_symbol(left_column.2), is_gear_symbol(right_column.2)];

        

    let pos_1 = is_symbol(left_column.0);
    let pos_2 = is_symbol(left_column.1);
    let pos_3 = is_symbol(left_column.2);
    let pos_4 = is_symbol(center_column.0);
    let pos_5 = is_symbol(center_column.1);
    let pos_6 = is_symbol(right_column.0);
    let pos_7 = is_symbol(right_column.1);
    let pos_8 = is_symbol(right_column.2);

    // Debug output
    //println!("{} {} {} {} {} {} {} {} {}",pos_1, pos_2, pos_3, pos_4, pos_5, pos_6, pos_7, pos_8, is_digit);
    return ((pos_1 || pos_2 || pos_3 || pos_4 || pos_5 || pos_6 || pos_7 || pos_8) && is_digit, false);
}

fn find_surrounding_gear_numbers(cur_pos: usize, cur_line: String, prev_line: String, next_line: String) -> Vec<i32> {
    let prev_char_array: Vec<char> = prev_line.chars().collect::<Vec<char>>().try_into().unwrap();
    let cur_char_array: Vec<char> = cur_line.chars().collect::<Vec<char>>().try_into().unwrap();
    let next_char_array: Vec<char> = next_line.chars().collect::<Vec<char>>().try_into().unwrap();

    if !is_gear_symbol(cur_char_array[cur_pos]) { return [].to_vec() }

    let check_prev = if prev_line.is_empty() { false } else { true };
    let check_next = if next_line.is_empty() { false } else { true };
    let check_left = if cur_pos <= 0 { false } else { true };
    let check_right = if cur_pos >= cur_line.len()-1 { false } else { true };

    let left_column = (
        if check_prev && check_left { prev_char_array[cur_pos-1]} else { '.' }, 
        if check_left { cur_char_array[cur_pos-1] } else { '.' }, 
        if check_next && check_left { next_char_array[cur_pos-1] } else { '.' }
    );
    let center_column = (
        if check_prev { prev_char_array[cur_pos] } else { '.' }, 
        if check_next { next_char_array[cur_pos] } else { '.' }
    );
    let right_column = (
        if check_prev && check_right { prev_char_array[cur_pos+1] } else { '.' }, 
        if check_right { cur_char_array[cur_pos+1] } else { '.' }, 
        if check_next && check_right { next_char_array[cur_pos+1] } else { '.' }
    );

    let mut surrounding_numbers: Vec<i32> = Vec::new();
    let mut end_pos = cur_pos-1;

    // Check the line above the current gear symbol
    println!("\nFirst {}, {}",center_column.0, end_pos <= cur_pos);
    if left_column.0.is_digit(10) { let num = build_number(cur_pos-1, prev_line.clone()); surrounding_numbers.push(num.0); end_pos = num.1 + 1  }
    if center_column.0.is_digit(10) && end_pos <= cur_pos { let num = build_number(cur_pos, prev_line.clone()); surrounding_numbers.push(num.0); end_pos = num.1 + 1 }
    if right_column.0.is_digit(10) && end_pos <= cur_pos+1 { surrounding_numbers.push(build_number(cur_pos+1, prev_line.clone()).0) }

    // end_pos = if check_left { cur_pos-1 } else { cur_pos };
    // Check left and right neighbors of current gear symbol
    if left_column.1.is_digit(10) { surrounding_numbers.push(build_number(cur_pos-1, cur_line.clone()).0) }
    if right_column.1.is_digit(10) { surrounding_numbers.push(build_number(cur_pos+1, cur_line.clone()).0) }

    end_pos = cur_pos-1;
    println!("Second {}, {}",center_column.1, end_pos <= cur_pos);
    // Check the line below the current gear symbol
    if left_column.2.is_digit(10) { let num = build_number(cur_pos-1, next_line.clone()); surrounding_numbers.push(num.0); end_pos = num.1 + 1 }
    if center_column.1.is_digit(10) && end_pos <= cur_pos { let num = build_number(cur_pos, next_line.clone()); surrounding_numbers.push(num.0); end_pos = num.1 + 1 }
    println!("cur: {}, end: {}", cur_pos,end_pos);
    if right_column.2.is_digit(10) && end_pos < cur_pos+1 { surrounding_numbers.push(build_number(cur_pos+1, next_line.clone()).0) }

    println!("{:?}", surrounding_numbers);
    return surrounding_numbers;
}

fn is_symbol(check: char) -> bool {
    return !check.is_digit(10) && check != '.';
}

fn is_gear_symbol(check: char) -> bool {
    return check == '*';
}

fn build_number(cur_pos: usize, cur_line: String) -> (i32, usize) {
    let line_array: Vec<char> = cur_line.chars().collect::<Vec<char>>().try_into().unwrap();
    let mut left_pos = cur_pos;
    let mut right_pos = cur_pos;
    while left_pos > 0 && line_array[left_pos-1].is_digit(10) { left_pos -= 1 };
    while right_pos < line_array.len() -1 && line_array[right_pos+1].is_digit(10) { right_pos += 1 };
    let number: String = line_array[left_pos..right_pos+1].iter().collect();
    match number.trim().parse::<i32>() {
        Ok(num) => { return (num, right_pos) },
        Err(_msg) => { return (0, right_pos) }
    }
}
