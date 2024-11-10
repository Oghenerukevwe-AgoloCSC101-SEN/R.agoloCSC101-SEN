use std::io;

fn main() {
    let (a, b, c) = get_coefficients();
    let discriminant = b * b - 4 * a * c;

    if discriminant > 0 {
        // Two distinct real roots
        let root1 = (-b as f64 + (discriminant as f64).sqrt()) / (2.0 * a as f64);
        let root2 = (-b as f64 - (discriminant as f64).sqrt()) / (2.0 * a as f64);
        println!("Two real roots: {} and {}", root1, root2);
    } else if discriminant == 0 {
        // One real root
        let root = -b as f64 / (2.0 * a as f64);
        println!("One real root: {}", root);
    } else {
       // No real roots
       println!("No real roots.");
    }
}

fn get_coefficients() -> (i32, i32, i32) {
    let mut input = String::new();
    
    println!("Enter value for a:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let a: i32 = input.trim().parse().expect("Invalid input");

    

    input.clear();
    println!("Enter value for b:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let b: i32 = input.trim().parse().expect("Invalid input");

    input.clear();
    println!("Enter value for c:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let c: i32 = input.trim().parse().expect("Invalid input");

    (a, b, c)
}
