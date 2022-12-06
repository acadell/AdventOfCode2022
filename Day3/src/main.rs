use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    //define input
    let all_lines = input.split("\n");

    let mut matched_items = String::from("");

    let mut sum = 0;

    for line in all_lines {

        let count = line.len() - 1; // len is fine here because the input will only be a-z + A-Z 
        let half = count / 2;

        let pocket1 = &line[..half];
        let pocket2 = &line[half..];

        let mut matching_item = ' ';
        for item in pocket1.chars() {
            
            if pocket2.contains(item) {
                matching_item = item;
            }
            
        }
        let item_score = if !matching_item.is_whitespace() {
            score(matching_item)
        }
        else {0};

        sum += item_score;
        
        println!("{line}");
        println!("{count}");
        println!("Pocket 1: {pocket1}");
        println!("Pocket 2: {pocket2}");
        println!("Score: {item_score}\n\n");
    }
    println!("Total Matched items: {matched_items}");
    println!("Score: {sum}");
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