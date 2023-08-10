use regex::Regex;


pub struct Reader {
    position: usize,
    tokens: Vec<String>
}

trait Readable {
    fn next(&mut self) -> Result<String, String>;
    fn peek(&self) -> Result<String, String>;
}

impl Readable for Reader {
    fn next(&mut self) -> Result<String, String> {
        let token = self.peek();
        self.position += 1;
        token
    }
    fn peek(&self) -> Result<String, String> {
        match self.tokens.get(self.position) {
            Some(x) => Ok(x.to_string()),
            _ => Err("Error reading data.".to_string())
        }
    }
}

#[derive(Debug)]
pub enum MalType {
    Number(i32),
    Symbol(String),
    List(Vec<MalType>),
    Nil,
    True,
    False,
}

pub fn read_str(x: String) -> Result<MalType, String> {
    let tokens = tokenize(x);
    let mut r = Reader { 
        position: 0, 
        tokens: tokens
    };
    read_form(&mut r)
}

fn tokenize(x: String) -> Vec<String> {
    let re = Regex::new(r#"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"#).unwrap();
    re.captures_iter(&x).map(|x| x[1].to_string()).collect()
}

fn read_form(x: &mut Reader) -> Result<MalType, String> {
    if x.peek()? == "(" {
        return read_list(x);
    }
    Ok(read_atom(x))
}

fn read_list(x: &mut Reader) -> Result<MalType, String> {
    let mut symbols: Vec<MalType> = vec![];
    x.next();
    while x.peek()? != ")" {
        symbols.push(read_form(x).unwrap());
        x.next();
    };
    Ok(MalType::List(symbols))
}

fn read_atom(x: &mut Reader) -> MalType {
    let token = x.peek().unwrap();

    match token.parse::<i32>() {
        Ok(x) => MalType::Number(x),
        _ => {
            match token.as_str() {
                "nil" => MalType::Nil,
                "true" => MalType::True,
                "false" => MalType::False,
                _ => MalType::Symbol(token)
            }
        }
    }
}