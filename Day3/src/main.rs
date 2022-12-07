use std::{fs, collections::HashSet};

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    //define input
    let all_lines = input.split("\n");

    let mut badges: Vec<char> = Vec::new();
    let mut sum = 0;
    let mut counter = 1;

    let mut lastline = String::from("");
    for line in all_lines {

        if !lastline.is_empty(){

            lastline = find_common_item1(&lastline, line);

            if counter % 3 == 0 {
                let badge = lastline.chars().next().unwrap();
    
                badges.push(badge);
                lastline.clear();
                counter = 0;
                print!("Badge: {badge} reset: ");
            }
        }
        else{
            lastline = line.to_owned();
        }
        counter+=1;
        println!("LAST LINE: {lastline}");

        


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

    let mut badge_sum = 0;
    for badge in badges {
        badge_sum += score(badge);
    }

    println!("Score: {sum}");
    println!("Badge score: {badge_sum}");
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
    matching_item
}


fn find_common_item1(pocket1: &str, pocket2: &str) -> String{
    let mut matching_item = String::from("");
    for item in pocket1.chars() {
    
        if pocket2.contains(item) {
            matching_item.push(item)
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