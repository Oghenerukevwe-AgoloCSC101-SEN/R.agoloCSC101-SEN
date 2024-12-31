struct Commissioner {
    name: String,
    ministry: String,
    geopolitical_zone: String,
}

fn main() {
    
    let names = vec![
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbonna",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etiye",
    ];

    
    let ministries = vec![
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    
    let geopolitical_zones = vec![
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    
    let mut commissioners: Vec<Commissioner> = Vec::new();

    for i in 0..names.len() {
        commissioners.push(Commissioner {
            name: names[i].to_string(),
            ministry: ministries[i].to_string(),
            geopolitical_zone: geopolitical_zones[i].to_string(),
        });
    }

    
    println!(
        "{:<5} {:<30} {:<20} {:<15}",
        "S/N", "Name of Commissioner", "Ministry", "Geopolitical Zone"
    );
    println!("{:-<80}", "");
    for (index, commissioner) in commissioners.iter().enumerate() {
        println!(
            "{:<5} {:<30} {:<20} {:<15}",
            index + 1,
            commissioner.name,
            commissioner.ministry,
            commissioner.geopolitical_zone
        );
    }
}

