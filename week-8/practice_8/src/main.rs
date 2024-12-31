fn main() {
    // initialize a mutuable tuple
    let mut mountain_heights = ("Everest", 8848, "Fishtall", 6993);

    println!("Original tuple = {:?}", mountain_heights);

    // change 3rd and 4th element of a mutable tuple
    mountain_heights.2 = "Lhotse";
    mountain_heights.3 = 8516;

    println!("Change tuple = {:?}", mountain_heights);
}
