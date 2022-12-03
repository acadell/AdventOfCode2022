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
        
        let moveToMake = calculate_required_input(elf_input, my_input);
        
        //calculate score
        let score = calculate_score(elf_input, &moveToMake);

        // println!("elf input: {elf_input}");
        // println!("my input {my_input}");
        // println!("round score {score}");
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

    let myRock = "A";
    let myPaper = "B";
    let myScissors = "C";

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

fn calculate_score_part1(elf_input : &str, my_input : &str) -> i32
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

fn calculate_required_input(elf_input : &str, code_input : &str) -> String
{
    let elfRock = "A";
    let elfPaper = "B";
    let elfScissors = "C";

    let lose = "X";
    let draw = "Y";
    let win = "Z";

    //Rock, Scissors, Paper
    let desiredMove = ["A","C","B"];
    let position = desiredMove.iter().position(|&s| s == elf_input).unwrap();

    let moveIndex = 
    if code_input == win
    {
      (position + 2) % 3 
    }
    else if code_input == draw {
        position
    }
    else if code_input == lose {
        (position + 1) % 3
    }
    else {
        panic!("INVALID INPUT")
    };

    let moveToMake = desiredMove[moveIndex];
    println!("move to make: {moveToMake}");
    moveToMake.to_string()
}