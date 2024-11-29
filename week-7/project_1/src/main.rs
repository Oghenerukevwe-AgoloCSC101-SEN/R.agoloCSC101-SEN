use std::io;

fn main() {
    println!("Select the calculation you want to perform:");
    println!("1. Area of Trapezium");
    println!("2. Area of Rhombus");
    println!("3. Area of Parallelogram");
    println!("4. Area of Cube");
    println!("5. Volume of Cylinder");

    let choice = get_input("Enter your choice (1-5):");

    match choice.trim() {
        "1" => {
            let height: f64 = get_numeric_input("Enter the height of the trapezium:");
            let base1: f64 = get_numeric_input("Enter the first base of the trapezium:");
            let base2: f64 = get_numeric_input("Enter the second base of the trapezium:");
            println!("Area of Trapezium: {:.2}", area_of_trapezium(height, base1, base2));
        }
        "2" => {
            let diagonal1: f64 = get_numeric_input("Enter the first diagonal of the rhombus:");
            let diagonal2: f64 = get_numeric_input("Enter the second diagonal of the rhombus:");
            println!("Area of Rhombus: {:.2}", area_of_rhombus(diagonal1, diagonal2));
        }
        "3" => {
            let base: f64 = get_numeric_input("Enter the base of the parallelogram:");
            let altitude: f64 = get_numeric_input("Enter the altitude of the parallelogram:");
            println!("Area of Parallelogram: {:.2}", area_of_parallelogram(base, altitude));
        }
        "4" => {
            let side: f64 = get_numeric_input("Enter the length of the side of the cube:");
            println!("Area of Cube: {:.2}", area_of_cube(side));
        }
        "5" => {
            let radius: f64 = get_numeric_input("Enter the radius of the cylinder:");
            let height: f64 = get_numeric_input("Enter the height of the cylinder:");
            println!("Volume of Cylinder: {:.2}", volume_of_cylinder(radius, height));
        }
        _ => {
            println!("Invalid choice. Please restart the program and select a valid option.");
        }
    }
}

fn area_of_trapezium(height: f64, base1: f64, base2: f64) -> f64 {
    height / 2.0 * (base1 + base2)
}

fn area_of_rhombus(diagonal1: f64, diagonal2: f64) -> f64 {
    0.5 * diagonal1 * diagonal2
}

fn area_of_parallelogram(base: f64, altitude: f64) -> f64 {
    base * altitude
}

fn area_of_cube(side: f64) -> f64 {
    6.0 * side.powi(2)
}

fn volume_of_cylinder(radius: f64, height: f64) -> f64 {
    std::f64::consts::PI * radius.powi(2) * height
}

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

fn get_numeric_input(prompt: &str) -> f64 {
    loop {
        let input = get_input(prompt);
        match input.parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("Invalid input. Please enter a valid number."),
        }
    }
}
