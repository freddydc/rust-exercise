// quiz3.rs

// This quiz tests:
// - Generics
// - Traits
// An imaginary magical school has a new report card generation system written in Rust!
// Currently the system only supports creating report cards where the student's grade
// is represented numerically (e.g. 1.0 -> 5.5).
// However, the school also issues alphabetical grades (A+ -> F-) and needs
// to be able to print both types of report card!

use std::fmt;

pub fn run_quiz3() {
    let numeric_card = ReportCard {
        grade: 2.1,
        student_age: 12,
        student_name: "Goat".to_string(),
    };

    let alphabetic_card = ReportCard {
        student_age: 11,
        student_name: "Sheep".to_string(),
        grade: "A+",
    };

    println!("{}", numeric_card.print());
    println!("{}", alphabetic_card.print());
}

pub struct ReportCard<T> {
    pub grade: T,
    pub student_name: String,
    pub student_age: u8,
}

impl<T: fmt::Display> ReportCard<T> {
    pub fn print(&self) -> String {
        let ReportCard {
            grade,
            student_name,
            student_age,
        } = self;

        format!(
            "{} ({}) - achieved a grade of {}",
            student_name, student_age, grade
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Goat".to_string(),
            student_age: 12,
        };
        assert_eq!(report_card.print(), "Goat (12) - achieved a grade of 2.1");
    }

    #[test]
    fn generate_alphabetic_report_card() {
        let report_card = ReportCard {
            grade: "A+",
            student_name: "Sheep".to_string(),
            student_age: 11,
        };
        assert_eq!(report_card.print(), "Sheep (11) - achieved a grade of A+");
    }
}
