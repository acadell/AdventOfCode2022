use std::{fs};

fn main() {
    let input = fs::read_to_string("input.txt")
    .expect("Should have been able to read the file");

    //define input
    let all_lines = input.lines();
    let buffer_size = 14;

    
    for line in all_lines {
        let mut buffer: Vec<char> = Vec::new();
        let characters: Vec<char> = line.chars().collect();
        
        for i in 0..characters.len() {

            let next_char = characters[i];
            
            
            
            if buffer.len() >= buffer_size {

                //check if buffer is all unique
                let all_unique = is_all_unique_characters(buffer_size, &buffer);

                if all_unique {
                    println!("Unique Buffer: {:?} after {i}", buffer);
                    break;
                }

                buffer.pop();
            }
            buffer.insert(0, next_char);
        }
    }
}

fn is_all_unique_characters(buffer_size: usize, buffer: &Vec<char>) -> bool {
    let mut all_unique = true;
    let mut current_chars: Vec<char> = Vec::new();
    for j in 0..buffer_size {
        let buffer_char = buffer[j];
    
        if current_chars.contains(&buffer_char) {
            all_unique = false;
            break;
        }
        current_chars.push(buffer_char);
    }
    current_chars.clear();
    all_unique
}
