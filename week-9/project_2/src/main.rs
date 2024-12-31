use std::fs::File;
use std::io::{self, Write};

#[derive(Debug)]
struct Student {
    name: String,
    matric_number: String,
    department: String,
    level: u32,
}

fn main() -> io::Result<()> {
    // Step 1: Create a vector of students
    let students = vec![
        Student {
            name: String::from("Oluchi Mordi"),
            matric_number: String::from("ACC10211111"),
            department: String::from("Accounting"),
            level: 300,
        },
        Student {
            name: String::from("Adams Aliyu"),
            matric_number: String::from("ECO10111001"),
            department: String::from("Economics"),
            level: 400,
        },
        Student {
            name: String::from("Shania Bolade"),
            matric_number: String::from("CSC10328283"),
            department: String::from("Computer"),
            level: 200,
        },
        Student {
            name: String::from("Adekunle Gold"),
            matric_number: String::from("EEE11020202"),
            department: String::from("Electrical"),
            level: 300,
        },
        Student {
            name: String::from("Blanca Edemon"),
            matric_number: String::from("MEE10202001"),
            department: String::from("Mechanical"),
            level: 100,
        },
    ];

    // Step 2: Display the student details
    println!("PAU SMIS\n");
    println!("{:<20} {:<15} {:<15} {:<5}", "Student Name", "Matric. Number", "Department", "Level");
    for student in &students {
        println!(
            "{:<20} {:<15} {:<15} {:<5}",
            student.name, student.matric_number, student.department, student.level
        );
    }

    // Step 3: Write the details to a file
    let mut file = File::create("student_details.txt")?;
    writeln!(file, "PAU SMIS\n")?;
    writeln!(file, "{:<20} {:<15} {:<15} {:<5}", "Student Name", "Matric. Number", "Department", "Level")?;
    for student in students {
        writeln!(
            file,
            "{:<20} {:<15} {:<15} {:<5}",
            student.name, student.matric_number, student.department, student.level
        )?;
    }

    println!("\nDetails have been saved to 'student_details.txt'.");
    Ok(())
}
