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
            
            // let mut start_index = i;
            // let mut end_index = start_index + buffer_size;
            // let next_index = end_index+1;
            let next_char = characters[i];
            
            
            
            if buffer.len() >= buffer_size {

                //check if buffer is all unique
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

                if all_unique {
                    println!("Unique Buffer: {:?} after {i}", buffer);
                    break;
                }

                // if !buffer.contains(&next_char) {
                    
                //     println!("found it at {i} with {next_char}");
                //     println!("{:?}", buffer);
                    
                // }
                buffer.pop();
            }
            buffer.insert(0, next_char);
            
           
           
           
         
        }
       

    }
}
