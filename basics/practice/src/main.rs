enum Grade {
    Letter(char),
    Percentage(u8),
    Passfail(bool)
}

impl Grade {
    fn evaluate(&self) -> &'static str {
        match self {
            Grade::Letter(a) => match a {
                'A' => "Excelent",
                'B' => "Good job",
                'C' => "Needs improvement",
                _ => "Invalid letter"
                
            },
            Grade::Percentage(a) => {
                if *a >= 90 {
                    "Outstanding score"
                }else if *a >= 80 {
                    "Well done!"
                } else if *a >= 70 {
                    "Solid performance"
                } else if *a >= 60 {
                    "Passing but can do better"
                } else {
                    "Failing"
                }
            },
            Grade::Passfail(a) => {
                if *a {
                    "Passed the course"
                } else {
                    "Failed the course"
                }
            }
                
            }
        }
    }


fn main() {
    let one = Grade::Letter('A');
    let two = Grade::Percentage(95);
    let three = Grade::Passfail(false);

    let range_s = 1..=100;

    println!("{}", one.evaluate());
    println!("{}", two.evaluate());
    println!("{}", three.evaluate());
}