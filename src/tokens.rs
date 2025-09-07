#[derive(Debug, PartialEq, Clone)]
pub enum Tokens {
    RED,
    GREEN,
    BLUE,
    NEWLINE,
    OPENBRACKET,
    CLOSEBRACKET,
    REPEAT,
    VALUE(i32 ),
}