fn main() {
    let principal: f64 = 520_000_000.0; // Principal loan amount in Naira
    let rate: f64 = 10.0; // Interest rate in percentage
    let years: u32 = 5; // Number of years

    // Calculate the future value (A) using compound interest formula
    let future_value = principal * (1.0 + rate / 100.0).powf(years as f64);

    // Calculate the compound interest
    let compound_interest = future_value - principal;

    println!("The future value after {} years is: N{:.2}", years, future_value);
    println!("The compound interest after {} years is: N{:.2}", years, compound_interest);
}
