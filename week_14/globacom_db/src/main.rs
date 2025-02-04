use std::io;

fn main() {
    println!("Enter your role (Admin, ProjectManager, Employee, Customer, Vendor):");
    
    let mut role = String::new();
    io::stdin().read_line(&mut role).expect("Failed to read input");
    
    let role = role.trim().to_lowercase();

    match role.as_str() {
        "admin" => show_database_structure(),
        "projectmanager" => show_project_table(),
        "employee" => show_staff_table(),
        "customer" => show_customer_table(),
        "vendor" => show_data_plan_table(),
        _ => println!("Invalid role! Please enter a valid role."),
    }
}

fn show_database_structure() {
    println!("Database Structure:");
    println!("--------------------------------");
    println!("Tables: [Projects, Staff, Customers, DataPlans]");
}

fn show_project_table() {
    println!("Project Table Structure:");
    println!("--------------------------------");
    println!("Columns: [Project_ID, Name, Budget, Duration]");
}

fn show_staff_table() {
    println!("Staff Table Structure:");
    println!("--------------------------------");
    println!("Columns: [Staff_ID, Name, Position, Salary]");
}

fn show_customer_table() {
    println!("Customer Table Structure:");
    println!("--------------------------------");
    println!("Columns: [Customer_ID, Name, Contact, Purchases]");
}

fn show_data_plan_table() {
    println!("Data Plan Table Structure:");
    println!("--------------------------------");
    println!("Columns: [Data_ID, Data_Size, Duration, Price]");
}