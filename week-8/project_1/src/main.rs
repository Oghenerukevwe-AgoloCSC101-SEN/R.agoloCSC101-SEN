use std::collections::HashMap;

fn main() {
    // Define a mapping of roles to APS levels
    let mut aps_table: HashMap<&str, HashMap<&str, &str>> = HashMap::new();

    aps_table.insert("Office Administrator", HashMap::from([
        ("Intern", "APS 1-2"),
        ("Administrator", "APS 3-5"),
        ("Senior Administrator", "APS 5-8"),
        ("Office Manager", "EL1 8-10"),
        ("Director", "EL2 10-13"),
        ("CEO", "SES"),
    ]));

    aps_table.insert("Academic", HashMap::from([
        ("-", "APS 1-2"),
        ("Research Assistant", "APS 3-5"),
        ("PhD Candidate", "APS 5-8"),
        ("Post-Doc Researcher", "EL1 8-10"),
        ("Senior Lecturer", "EL2 10-13"),
        ("Dean", "SES"),
    ]));

    aps_table.insert("Lawyer", HashMap::from([
        ("Paralegal", "APS 1-2"),
        ("Junior Associate", "APS 3-5"),
        ("Associate", "APS 5-8"),
        ("Senior Associate 1-2", "EL1 8-10"),
        ("Senior Associate 3-4", "EL2 10-13"),
        ("Partner", "SES"),
    ]));

    aps_table.insert("Teacher", HashMap::from([
        ("Placement", "APS 1-2"),
        ("Classroom Teacher", "APS 3-5"),
        ("Snr Teacher", "APS 5-8"),
        ("Leading Teacher", "EL1 8-10"),
        ("Deputy Principal", "EL2 10-13"),
        ("Principal", "SES"),
    ]));

    // Example Input
    let role_category = "Lawyer";
    let specific_role = "Associate";

    // Check APS level
    if let Some(role_map) = aps_table.get(role_category) {
        if let Some(aps_level) = role_map.get(specific_role) {
            println!("The APS level for a {} in the {} category is {}.", specific_role, role_category, aps_level);
        } else {
            println!("Role '{}' not found in the '{}' category.", specific_role, role_category);
        }
    } else {
        println!("Role category '{}' not found.", role_category);
    }
}

