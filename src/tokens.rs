#[derive(Debug, PartialEq, Clone)]
pub enum Tokens {
    RED,
    GREEN,
    BLUE,
    NEWLINE,
    NONREPEATPOINT,
    OPENBRACKET,
    CLOSEBRACKET,
    REPEAT,
    VALUE(i32 ),
    DEBUG,
}