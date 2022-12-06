use std::{fs, collections::HashSet};

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    //define input
    let all_lines = input.split("\n");

    // let mut vec = Vec::new();
    let mut common_items : HashSet<char> = HashSet::new();
    let mut badges = "";
    let mut sum = 0;
    let mut badges_sum = 0;

    for line in all_lines {
        let line_items: HashSet<char>  = line.chars().collect();
        // vec.push(line);
        // if vec.len() == 3 {
        //     let bag1 ::HashSet = vec.pop().unwrap().chars().collect();
        //     let bag2 = vec.pop().unwrap().chars().collect();
        //     let bag3 = vec.pop().unwrap().chars().collect();
            
            
            
            
        // }

        if common_items.is_empty() {
            common_items = line_items;
        }
        else if common_items.len() == 1 {
            let a = common_items.drain().next().unwrap();
            badges_sum += score( a);
        }
        
        


        let count = line.len() - 1; // len is fine here because the input will only be a-z + A-Z 
        let half = count / 2;

        let matching_item = find_common_item(line, half);
        let item_score = if !matching_item.is_whitespace() {
            score(matching_item)
        }
        else {0};

        sum += item_score;

        println!("Whole pocket: {line}");
        println!("Number of items in pocket: {count}");
        
        println!("Score: {item_score}\n\n");
    }

    println!("Score: {sum}");
}

fn find_common_item(line: &str, half: usize) -> char {
    let pocket1 = &line[..half];
    let pocket2 = &line[half..];
    let mut matching_item = ' ';
    for item in pocket1.chars() {
    
        if pocket2.contains(item) {
            matching_item = item;
        }
    
    }
    println!("Pocket 1: {pocket1}");
    println!("Pocket 2: {pocket2}");
    matching_item
}



fn score(item : char) -> i32
{

    if item.is_uppercase() {
        item as i32 - 'A' as i32 + 27
    }
    else if item.is_lowercase() {
        item as i32 - 'a' as i32 + 1
    }
    else{0}

}