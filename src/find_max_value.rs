fn find_max_value(elves_calories: &Vec<u64>, max_arrays: [u64; 3]) -> u64{
    
    let mut current_max_local = 0;
    let current_max_1: u64 = max_arrays[0]; 
    let current_max_2: u64 = max_arrays[1];
    let mut iterator:usize = 0;

    for _number in elves_calories.iter(){
        if elves_calories[iterator] == current_max_1{
            //do nothing, as this max value has already been found
        } else if elves_calories[iterator] == current_max_2{
            //do nothing as this max value has already been found
        } else{
            if elves_calories[iterator] > current_max_local{
                current_max_local = elves_calories[iterator];
            }
        }        
        iterator+=1;
    }

    return current_max_local
}


pub fn find_3_highest_calories_carried(elves_calories: Vec<u64>) -> [u64; 3]{
    let mut return_max_calories: [u64; 3] = [0,0,0];
    let mut iterator: usize = 0;

    while return_max_calories[2] == 0{
        let current_max_local: u64 = find_max_value(&elves_calories, return_max_calories);
        return_max_calories[iterator] = current_max_local;
        iterator+=1;
    }

    return return_max_calories;
}