use crate::reader::MalType;

pub fn pr_str(x: &MalType) -> String {
    let ret = match x {
        MalType::Number(y) => y.to_string(),
        MalType::Symbol(y) => y.to_string(),
        MalType::List(y) => {
            let temp = y.iter().map(|z| pr_str(z)).reduce(|a, b| a + " " + b.as_str()).unwrap_or_default();
            format!("({})", temp)
        }
    };
    ret
}