use crate::reader::MalType;

pub fn pr_str(x: &MalType) -> String {
    let ret = match x {
        MalType::Number(y) => y.to_string(),
        MalType::Symbol(y) => y.to_string(),
        MalType::Nil => "nil".to_string(),
        MalType::True => "true".to_string(),
        MalType::False => "false".to_string(),
        MalType::Keyword(y) => y.to_string(),
        MalType::HMap(y) => {
            let res = y.iter()
                .map(|(a, b)| format!("{} {}", a, pr_str(b)))
                .reduce(|a, b| a + " " + b.as_str()).unwrap();         
            format!("{{{}}}", res)
        },
        MalType::Vector(y) => {
            let temp = y.iter().map(|z| pr_str(z)).reduce(|a, b| a + " " + b.as_str()).unwrap_or_default();
            format!("[{}]", temp)
        },
        MalType::List(y) => {
            let temp = y.iter().map(|z| pr_str(z)).reduce(|a, b| a + " " + b.as_str()).unwrap_or_default();
            format!("({})", temp)
        }
    };
    ret
}