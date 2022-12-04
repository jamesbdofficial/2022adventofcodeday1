// Advent of Code Day 1 - Calorie Counter.
// Autho - James Brozicevic - Doran.
pub mod file_to_vector;
pub mod find_max_value;

fn main() {    
    // Variable later used to total the amount carried by each elf.
    let mut calorie_number: u64 = 0;

    // Creates a vector carrying either a number or blank space
    let calorie_list_vector: Vec<String> = file_to_vector::text_to_vec();
    
    // The vector carrying the total of the calories each elf is carrying
    let mut elves_calories = Vec::<u64>::new();

    let mut iterator: usize = 0;

    for _number in calorie_list_vector.iter() {
        if calorie_list_vector[iterator].clone() as String == "" {
            elves_calories.push(calorie_number);
            calorie_number = 0;
            iterator+=1;
        } else { 
            let current_int: u64 = calorie_list_vector[iterator].trim().parse().unwrap();
            calorie_number = calorie_number + current_int;
            iterator+=1;
        }
    }

    let results_array: [u64; 3] = find_max_value::find_3_highest_calories_carried(elves_calories);
    let answer: u64 = results_array[0] + results_array[1] + results_array[2];
    println!("The total calories of the three elves carrying the most calories was: {}", answer);
}
