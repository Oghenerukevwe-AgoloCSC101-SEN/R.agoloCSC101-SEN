fn main() {
    // Initialization of a tuple with data type
    let datatype_tuple: (&str, f32, u8) = ("Rust", 3.14, 100);
    println!("Tuple contents (with data type) = {:?}", datatype_tuple);

    // Initialization of a tuple without specifying data type
    let no_datatype_tuple = ("Rust", "fun", 100);
    println!("Tuple contents (without data type) = {:?}", no_datatype_tuple);

    // Accessing tuple elements by index
    println!("Value at Index 0 = {}", datatype_tuple.0);
    println!("Value at Index 1 = {}", datatype_tuple.1);
    println!("Value at Index 2 = {}", datatype_tuple.2);
}
