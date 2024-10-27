fn main() {
    let p: f64 = 510_000.0; // Initial value of the TV in Naira
    let r: f64 = 5.0; // Depreciation rate in percentage
    let years: u32 = 3; // Number of years

    // Calculate the depreciated value using compound depreciation formula
    let final_value = p * (1.0 - r/ 100.0).powf(years as f64);

    println!("The value of the TV after {} years is: N{:.2}", years, final_value);
}
