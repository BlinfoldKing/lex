#[derive(Debug, PartialEq, Clone)]
pub enum Token<'a> {
    Comment,
    String(&'a str),
    Number(f64),
    Atom(&'a str),
    Operator(&'a str),
    Keyword(&'a str),
    Variable(&'a str),
    Wildcard(&'a str),
    Boolean(bool),
    List(Vec<Token<'a>>),
    Whitespace,
}
