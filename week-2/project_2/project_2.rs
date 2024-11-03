fn main() {
    // Sales data: (Item, Quantity, Amount)
    let sales_records = vec![
        ("Toshiba", 2, 450_000.00),
        ("Mac", 1, 1_500_000.00),
        ("HP", 3, 750_000.00),
        ("Dell", 3, 2_850_000.00),
        ("Acer", 1, 250_000.00),
    ];

    // Calculate the sum of the amounts
    let total_sum: f64 = sales_records.iter().map(|&(_, _, amount)| amount).sum();
    println!("Total Sales Sum: {:.2}", total_sum);

    // Calculate the average amount
    let count = sales_records.len() as f64;
    let average = total_sum / count;
    println!("Average Sales Amount: {:.2}", average);
}
