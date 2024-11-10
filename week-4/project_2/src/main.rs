use std::io;

fn main() {
    let (experience, age) = get_employee_info();
    let incentive = calculate_incentive(experience, age);
    println!("The annual incentive is: N{}", incentive);
}

fn get_employee_info() -> (bool, i32) {
    let mut input = String::new();
    
    println!("Is the employee experienced? (yes/no):");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let experience = input.trim().eq_ignore_ascii_case("yes");

    input.clear();
    println!("Enter the age of the employee:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let age: i32 = input.trim().parse().expect("Invalid input");

    (experience, age)
}

fn calculate_incentive(experience: bool, age: i32) -> i32 {
    if experience {
        if age >= 40 {
            1_560_000
        } else if age >= 30 {
            1_480_000
        } else if age < 28 {
            1_300_000
        } else {
            0
        }
    } else {
        100_000
    }
}


