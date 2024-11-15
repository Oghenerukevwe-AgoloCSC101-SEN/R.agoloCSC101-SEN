

use std::io::{self, Write};

struct Sibling {
    first_name: String,
    age: u32,
    gender: String,
    country_of_residence: String,
    relationship_status: Option<String>,
    additional_info: Option<AdditionalInfo>,
}

enum AdditionalInfo {
    Married {
        children: Vec<Child>,
    },
    Single(SingleStatus),
}

struct Child {
    name: String,
    age: u32,
    school_or_daycare: String,
}

enum SingleStatus {
    Student {
        university: String,
        course_of_study: String,
        year_of_study: u32,
        is_abroad: bool,
    },
    Employed {
        company_name: String,
        job_title: String,
        industry_sector: String,
        job_type: JobType,
    },
    InRelationship {
        partner_duration: u32,
        partner_first_name: String,
        lives_with_partner: bool,
    },
}

enum JobType {
    OnSite,
    Remote,
    Hybrid,
}

fn main() {
    let mut siblings = Vec::new();

    println!("Enter the number of siblings:");
    let num_siblings: usize = read_input().trim().parse().expect("Please enter a valid number");

    for _ in 0..num_siblings {
        println!("Enter the first name of the sibling:");
        let first_name = read_input();

        println!("Enter the age of the sibling:");
        let age: u32 = read_input().trim().parse().expect("Please enter a valid age");

        println!("Enter the gender of the sibling:");
        let gender = read_input();

        println!("Enter the country of residence:");
        let country_of_residence = read_input();

        let mut relationship_status = None;
        let mut additional_info = None;

        if age >= 18 {
            println!("Is the sibling married, single, or in a relationship?");
            relationship_status = Some(read_input());

            match relationship_status.as_deref() {
                Some("married") => {
                    let mut children = Vec::new();
                    println!("Does the sibling have any children? (yes/no)");
                    if read_input().trim().eq_ignore_ascii_case("yes") {
                        loop {
                            println!("Enter child's name:");
                            let child_name = read_input();

                            println!("Enter child's age:");
                            let child_age: u32 = read_input().trim().parse().expect("Please enter a valid age");

                            println!("Enter child's school or daycare:");
                            let school_or_daycare = read_input();

                            children.push(Child {
                                name: child_name,
                                age: child_age,
                                school_or_daycare,
                            });

                            println!("Add another child? (yes/no)");
                            if read_input().trim().eq_ignore_ascii_case("no") {
                                break;
                            }
                        }
                    }
                    additional_info = Some(AdditionalInfo::Married { children });
                }
                Some("single") => {
                    println!("Is the sibling a student or employed?");
                    match read_input().as_str() {
                        "student" => {
                            println!("Enter university:");
                            let university = read_input();

                            println!("Enter course of study:");
                            let course_of_study = read_input();

                            println!("Enter year of study:");
                            let year_of_study: u32 = read_input().trim().parse().expect("Please enter a valid year");

                            println!("Is the sibling studying abroad? (yes/no)");
                            let is_abroad = read_input().trim().eq_ignore_ascii_case("yes");

                            additional_info = Some(AdditionalInfo::Single(SingleStatus::Student {
                                university,
                                course_of_study,
                                year_of_study,
                                is_abroad,
                            }));
                        }
                        "employed" => {
                            println!("Enter company name:");
                            let company_name = read_input();

                            println!("Enter job title:");
                            let job_title = read_input();

                            println!("Enter industry sector:");
                            let industry_sector = read_input();

                            println!("Is the job on-site, remote, or hybrid?");
                            let job_type = match read_input().as_str() {
                                "remote" => JobType::Remote,
                                "hybrid" => JobType::Hybrid,
                                _ => JobType::OnSite,
                            };

                            additional_info = Some(AdditionalInfo::Single(SingleStatus::Employed {
                                company_name,
                                job_title,
                                industry_sector,
                                job_type,
                            }));
                        }
                        _ => (),
                    }
                }
                Some("in a relationship") => {
                    println!("Enter relationship duration (in years):");
                    let partner_duration: u32 = read_input().trim().parse().expect("Please enter a valid number");

                    println!("Enter partner's first name:");
                    let partner_first_name = read_input();

                    println!("Do they live together? (yes/no)");
                    let lives_with_partner = read_input().trim().eq_ignore_ascii_case("yes");

                    additional_info = Some(AdditionalInfo::Single(SingleStatus::InRelationship {
                        partner_duration,
                        partner_first_name,
                        lives_with_partner,
                    }));
                }
                _ => (),
            }
        } else {
            println!("Has the sibling completed WAEC? (yes/no)");
            if read_input().trim().eq_ignore_ascii_case("yes") {
                println!("Enter year WAEC was completed:");
                
            } else {
                println!("Will they take WAEC soon? (yes/no)");
                if read_input().trim().eq_ignore_ascii_case("yes") {
                    println!("Enter planned year for WAEC:");
                    
                }
            }
        }

        siblings.push(Sibling {
            first_name,
            age,
            gender,
            country_of_residence,
            relationship_status,
            additional_info,
        });
    }

    println!("Sibling details gathered successfully.");
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

