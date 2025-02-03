// Define a struct for Laptop
struct Laptop {
    brand: String,
    price: u32,
}

impl Laptop {
    // Method to calculate total cost based on quantity
    fn total_cost(&self, quantity: u32) -> u32 {
        self.price * quantity
    }
}

fn main() {
    // Create instances for each laptop brand
    let hp = Laptop {
        brand: String::from("HP"),
        price: 650_000,
    };
    let ibm = Laptop {
        brand: String::from("IBM"),
        price: 755_000,
    };
    let toshiba = Laptop {
        brand: String::from("Toshiba"),
        price: 550_000,
    };
    let dell = Laptop {
        brand: String::from("Dell"),
        price: 850_000,
    };

    // Each brand purchased 3 laptops
    let quantity = 3;

    // Calculate the total cost
    let total_cost = hp.total_cost(quantity)
        + ibm.total_cost(quantity)
        + toshiba.total_cost(quantity)
        + dell.total_cost(quantity);

    // Print the total cost
    println!("Total cost for 3 laptops from each brand: ₦{}", total_cost);
}
