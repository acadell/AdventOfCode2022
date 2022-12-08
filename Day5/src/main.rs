use std::{fs, num};

fn main() {
    let input = fs::read_to_string("input.txt")
    .expect("Should have been able to read the file");

    //define input
    let all_lines = input.lines();
    

    let mut crates: Vec<Vec<char>> = Vec::new();
    let mut rows: Vec<&str> = Vec::new();

    let mut read_diagram = true;
    for line in all_lines {

        if read_diagram {
            //initialize all crate bins
            if crates.is_empty() {
                let line_length = line.len();
                let num_stacks = line_length / 4 + 1;

                for _i in 0..num_stacks {
                    let stack: Vec<char> = Vec::new();
                    crates.push(stack);
                }

            }



            //println!("{line}");
            //put crates into the correct stack
            let chars: Vec<char> = line.chars().collect();

            for i in 0..chars.len() {

                if i %4 == 0  {
                    let j = i / 4;
                    let package = chars[i + 1];
                    if package.is_ascii_alphabetic() {
                        crates[j].push(package);
                        // println!("Push: {package}");
                    }
                }
                
            }

            rows.push(line);



            if line.trim().is_empty() {
                read_diagram = false;
                for i in 0..crates.len() {
                    crates[i].reverse();
                }

            }
        }
        else {
            let instructions: Vec<&str> = line.split(" ").collect();
            let num_to_move = instructions[1].parse::<usize>().unwrap();
            let from_index = instructions[3].parse::<usize>().unwrap() - 1;
            let to_index = instructions[5].parse::<usize>().unwrap() - 1;

            //CrateMover 9000
            // for i in 0..num_to_move {
            //     let top_crate = crates[from_index].pop().unwrap();
            //     crates[to_index].push(top_crate);
            //     // println!("Move: {top_crate}")
            // }
            // println!("instruction done");

            //CrateMover 9001
            let mut crate_stack: Vec<char> = Vec::new();
            for i in 0..num_to_move {
                let top_crate = crates[from_index].pop().unwrap();
                crate_stack.push(top_crate);                
                // println!("Move: {top_crate}")
            }
            
            for _i in 0..crate_stack.len() {
                crates[to_index].push(crate_stack.pop().unwrap());
            }

        }
    }

    let line_length = rows[0].len();
    let num_stacks = line_length / 4 + 1;

    println!("Number of stacks: {num_stacks}");


    for boxes in crates {
        // for pkg in boxes {
        //     print!("{pkg} ");
        // }
        // println!("");
        let top_box = boxes.last().unwrap();
        print!("{top_box}")
    }

}
