use std::collections::HashMap;

fn main() {
    // Define a list of candidates with their years of programming experience
    let candidates = vec![
        ("Alice", 5),
        ("Bob", 8),
        ("Charlie", 3),
        ("Diana", 10),
        ("Eve", 7),
    ];

    // Find the candidate with the highest years of experience
    let mut most_experienced = ("", 0);

    for candidate in &candidates {
        if candidate.1 > most_experienced.1 {
            most_experienced = *candidate;
        }
    }

    println!(
        "The candidate with the highest years of programming experience is {} with {} years.",
        most_experienced.0, most_experienced.1
    );
}

