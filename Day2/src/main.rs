use std::fs;

fn main() {

    let input = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");


    

    //define input
    let mut allGames = input.split("\n");
    let game = allGames.next().unwrap();
    
    // let mut game_inputs = game.split(" ");

    let mut total_score = 0;

    let score = calculate_score("A", "Y");
    println!("{score}");
    let score = calculate_score("B", "X");
    println!("{score}");
    let score = calculate_score("C", "Z");
    println!("{score}");

    // for game in allGames {
        
    //     let mut game_inputs = game.split(" ");
    //     let elf_input = game_inputs.next().unwrap();
    //     let my_input = game_inputs.next().unwrap();
        
    //     //calculate score
    //     let score = calculate_score(elf_input, my_input);

    //     // println!("elf input: {elf_input}");
    //     // println!("my input {my_input}");
    //     // println!("round score {score}");
    //     total_score += score;
    // }

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
        match my_input{
            myPaper => winValue + paperValue,
            myScissors => lossValue + scissorsValue,
            myRock => drawValue + rockValue
        }
    }
    else if elf_input == elfPaper {
        match my_input{
            myPaper => drawValue + paperValue,
            myScissors => winValue + scissorsValue,
            myRock => lossValue + rockValue
        }
    }
    else { //scissors
        match my_input{
            myPaper => lossValue + paperValue,
            myScissors => drawValue + scissorsValue,
            myRock => winValue + rockValue
        }
    };

    println!("score: {newscore}, {elf_input},{my_input}");
    newscore

}
