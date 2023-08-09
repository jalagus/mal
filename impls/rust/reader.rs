use regex::Regex;


pub struct Reader {
    position: usize,
    tokens: Vec<String>
}

trait Readable {
    fn next(&mut self) -> String;
    fn peek(&self) -> String;
}

impl Readable for Reader {
    fn next(&mut self) -> String {
        let old_pos = self.position;
        self.position += 1;
        self.tokens[old_pos].to_string()
    }
    fn peek(&self) -> String {
        self.tokens[self.position].to_string()
    }
}

pub enum MalType {
    Number(i32),
    Symbol(String),
    List(Vec<MalType>)
}

pub fn read_str(x: String) -> MalType {
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

fn read_form(x: &mut Reader) -> MalType {
    if x.peek() == "(" {
        return read_list(x);
    }
    read_atom(x)
}

fn read_list(x: &mut Reader) -> MalType {
    let mut symbols: Vec<MalType> = vec![];
    while x.next() != ")" {
        symbols.push(read_form(x));
    };
    MalType::List(symbols)
}

fn read_atom(x: &mut Reader) -> MalType {
    let token = x.peek();
    match token.parse::<i32>() {
        Ok(x) => MalType::Number(x),
        _ => MalType::Symbol(token)
    }
}