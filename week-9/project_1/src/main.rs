use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    
    let categories = vec![
        ("Lager", vec!["33 Export", "Desperados", "Goldberg", "Gulder", "Heineken", "Star"]),
        ("Stout", vec!["Legend", "Turbo King", "Williams"]),
        ("Non-Alcoholic", vec!["Maltina", "Amstel Malta", "Malta Gold", "Fayrouz"]),
    ];

    
    let mut file = File::create("nigerian_breweries.txt")?;

    
    writeln!(file, "High-Quality Categories of Drinks\n")?;
    for (category, items) in categories {
        writeln!(file, "{}:", category)?;
        for item in items {
            writeln!(file, "- {}", item)?;
        }
        writeln!(file)?;
    }

    println!("Data successfully written to 'nigerian_breweries.txt'");
    Ok(())
}

