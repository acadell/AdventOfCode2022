use std::fs;


fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");
    let binnedInput = input.split("\n\n");

    let mut largestCalories = 0;
    let mut elves = Vec::new();

    let mut numCalories = 0;
    for bin in binnedInput {
        numCalories = 0;
        let elfPockets = bin.split("\n");
        for calories in elfPockets {
            let number : i32 = calories.parse().unwrap();
            numCalories+= number;
        }
        
        elves.push(numCalories);
        
        if numCalories > largestCalories {
            largestCalories = numCalories;
        }
        println!("Elf Calories: {numCalories}");
    }

    println!("-------------------------------------------");
    println!("Largest calories: {largestCalories}");
    println!("New way:");
    elves.sort();
    
    let largest = elves.pop().unwrap();
    let secondLargest = elves.pop().unwrap();
    let thirdLargest = elves.pop().unwrap();

    println!("Largest Calories: {largest}");
    println!("Second Largest Calories: {secondLargest}");
    println!("Third Largest Calories: {thirdLargest}");
    
    let topThree = largest + secondLargest + thirdLargest;
    println!("Total top 3 {topThree}");
    
}
