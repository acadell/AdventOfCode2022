use std::fs;

fn main() {

    let input = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    //define input
    let all_lines = input.lines();

    let mut fully_contianed_zones = 0;
    let mut overlapping_zones = 0;
    //Check all lines
    for line in all_lines {

        let elves_zones : Vec<&str> = line.split(',').collect();
        let first_elf_assignment = elves_zones[0];
        let second_elf_assignment = elves_zones[1];

        let first_elf_zones: Vec<&str> = first_elf_assignment.split('-').collect();
        let second_elf_zones: Vec<&str> = second_elf_assignment.split('-').collect();
       
        let lower_zone_elf1 = first_elf_zones[0].parse::<i32>().unwrap();
        let upper_zone_elf1 = first_elf_zones[1].parse::<i32>().unwrap();

        let lower_zone_elf2 = second_elf_zones[0].parse::<i32>().unwrap();
        let upper_zone_elf2 = second_elf_zones[1].parse::<i32>().unwrap();

        fully_contained(lower_zone_elf1, lower_zone_elf2, upper_zone_elf1, upper_zone_elf2, &mut fully_contianed_zones);
        overlapping(lower_zone_elf1, lower_zone_elf2, upper_zone_elf1, upper_zone_elf2, &mut overlapping_zones);

    }

    println!("fully contained zones: {fully_contianed_zones}");
    println!("overlapping zones: {overlapping_zones}");

}

fn fully_contained(lower_zone_elf1: i32, lower_zone_elf2: i32, upper_zone_elf1: i32, upper_zone_elf2: i32, fully_contianed_zones: &mut i32) {
    if lower_zone_elf1 <= lower_zone_elf2  && upper_zone_elf1 >= upper_zone_elf2 {
        *fully_contianed_zones +=1;
    } else if lower_zone_elf1 >= lower_zone_elf2  && upper_zone_elf1 <= upper_zone_elf2 {
        *fully_contianed_zones += 1;
    }
}

fn overlapping(lower_zone_elf1: i32, lower_zone_elf2: i32, upper_zone_elf1: i32, upper_zone_elf2: i32, overlap: &mut i32) {

    let _ovrlap = if upper_zone_elf1 < lower_zone_elf2 || lower_zone_elf1 > upper_zone_elf2 {
        false
    } else if upper_zone_elf2 < lower_zone_elf1 || lower_zone_elf2 > upper_zone_elf1 {
        false
    }
    else
    {
        *overlap+=1;
        true
    };

}
