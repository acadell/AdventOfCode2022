use std::fs;

fn main() {

    let input = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");


    

    //define input
    let mut allGames = input.split("\n");
    
    
    // let mut game_inputs = game.split(" ");

    let mut total_score = 0;

    // let score = calculate_score("A", "Y");
    // println!("{score}");
    // let score = calculate_score("B", "X");
    // println!("{score}");
    // let score = calculate_score("C", "Z");
    // println!("{score}");

    for game in allGames {
        
        let mut game_inputs = game.split(" ");
        let elf_input = game_inputs.next().unwrap();
        let my_input = game_inputs.next().unwrap();
        
        //calculate score
        let score = calculate_score(elf_input, my_input);

        println!("elf input: {elf_input}");
        println!("my input {my_input}");
        println!("round score {score}");
        total_score += score;
    }

    println!("Total score: {total_score}");
}

fn calculate_score(elf_input : &str, my_input : &str) -> i32
{

    //define key
    let elfRock = "A";
    let elfPaper = "B";
    let elfScissors = "C";

    let myRock = "X";
    let myPaper = "Y";
    let myScissors = "Z";

    let rockValue = 1;
    let paperValue = 2;
    let scissorsValue = 3;

    let lossValue = 0;
    let drawValue = 3;
    let winValue = 6;

    
    let newscore = if elf_input == elfRock
    {
        if my_input == myPaper {winValue + paperValue}
        else if my_input == myScissors {lossValue + scissorsValue}
        else if my_input == myRock {drawValue + rockValue}
        else {panic!("INVALID INPUT")}
    }
    else if elf_input == elfPaper {
        if my_input == myPaper {drawValue + paperValue}
        else if my_input == myScissors {winValue + scissorsValue}
        else if my_input == myRock {lossValue + rockValue}
        else {panic!("INVALID INPUT")}

    }
    else { //scissors
        if my_input == myPaper {lossValue + paperValue}
        else if my_input == myScissors {drawValue + scissorsValue}
        else if my_input == myRock {winValue + rockValue}
        else {panic!("INVALID INPUT")}

    };

    newscore

}