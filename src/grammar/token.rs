#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Comment(&'a str),
    String(&'a str),
    Number(f64),
    Atom(&'a str),
    Operator(&'a str),
    Keyword(&'a str),
    Variable(&'a str),
    Wildcard(&'a str),
    Boolean(bool),
    List(Vec<Token<'a>>),
}
