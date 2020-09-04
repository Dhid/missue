use crate::{Issue, Status};
use colored::*;

const DEFAULT_SPACE: &str = " ";

pub struct Printer {}

impl Printer {
    /// Prints the Issue on standard output
    pub fn std_print(issue: &Issue) {
        match issue.status {
            Status::OPEN => {
                print!("{}", issue.name.on_red());
            },
            Status::CLOSED => {
                print!("{}", issue.name.green());
            }
            Status::DOING => {
                print!("{}", issue.name.on_blue());
            }
        }
        println!(" ({})", issue.creation_time.to_rfc2822());
        if let Some(descr) = &issue.description {
            println!("{}{}", DEFAULT_SPACE, descr);
        }
        println!("{}{} {}", DEFAULT_SPACE,"STATUS:".purple() ,issue.status);
        println!("");
    }

    pub fn std_print_string(issue_str: String) {
        let issue = Issue::from(issue_str);
        Printer::std_print(&issue);
    }
}
