use std::collections::HashMap;
use std::io;

fn main() {
    
    let menu = HashMap::from([
        ("P", ("Poundo Yam/Edinkaiko Soup", 3200)),
        ("F", ("Fried Rice & Chicken", 3000)),
        ("A", ("Amala & Ewedu Soup", 2500)),
        ("E", ("Eba & Egusi Soup", 2000)),
        ("W", ("White Rice & Stew", 2500)),
    ]);

    let mut total = 0;
    loop {
        println!("Menu:");
        for (code, (item, price)) in &menu {
            println!("{} = {} - N{}", code, item, price);
        }
        println!("Enter the food code (or type 'done' to finish):");

        let mut food_code = String::new();
        io::stdin().read_line(&mut food_code).unwrap();
        let food_code = food_code.trim().to_uppercase();

        if food_code == "DONE" {
            break;
        }

        if let Some((item, price)) = menu.get(food_code.as_str()) {
            println!("How many portions of {} do you want?", item);

            let mut quantity = String::new();
            io::stdin().read_line(&mut quantity).unwrap();
            let quantity: i32 = quantity.trim().parse().unwrap_or(0);

            total += price * quantity;
        } else {
            println!("Invalid food code. Please try again.");
        }
    }

    
    if total > 10000 {
        let discount = (total as f32 * 0.05) as i32;
        total -= discount;
        println!("A discount of 5% has been applied. You saved N{}.", discount);
    }

    println!("Your total bill is N{}.", total);
}
