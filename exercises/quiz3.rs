// quiz3.rs
//
// This quiz tests:
// - Generics
// - Traits
// 一个假想的魔法学校有一个用Rust编写的新的成绩单生成系统！目前系统仅支持创建成绩以数字形式表示的成绩单（例如1.0 -> 5.5）。然而，学校还发放字母等级成绩单（A+ -> F-），并需要能够打印两种类型的成绩单！
//
// 请在 ReportCard 结构体和 impl 代码块中进行必要的代码更改，以支持字母等级成绩单。在第二个测试中将 Grade 改为 "A+"，以展示您的更改支持字母等级。
//
// Execute `rustlings hint quiz3` or use the `hint` watch subcommand for a hint.

// I AM DONE

pub struct ReportCard<T> {
    pub grade: T,
    pub student_name: String,
    pub student_age: u8,
}

impl<T: std::fmt::Display> ReportCard<T> {  //加上可以格式化输出的类型约束
    pub fn print(&self) -> String {
        format!("{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        // TODO: Make sure to change the grade here after you finish the exercise.
        let report_card = ReportCard {
            grade: "A+",
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
