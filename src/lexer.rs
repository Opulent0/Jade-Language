extern crate regex;

//use regex::Regex;
use std::fs::File;
use std::io::Read;
use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f\r]+")] // Ignore this regex pattern between tokens
pub enum Token {
    // KEYWORDS
    #[token("var")]
    Var,
    #[token("const")]
    Const,
    #[token("sink")]
    Sink,

    // CONTROL SYMBOLS
    #[token("=")]
    Equals,
    #[token(";")]
    Semicolon,
    #[token("{")]
    OpenBrace,
    #[token("}")]
    CloseBrace,
    #[token("(")]
    OpenParen,
    #[token(")")]
    CloseParen,

    // OPERATORS:
    // Math
    #[token("+")]
    Addition,
    #[token("-")]
    Subtraction,
    #[token("*")]
    Multiplication,
    #[token("/")]
    Division,
    #[token("**")]
    Exponent,
    #[token("^/")]
    Root,

    // ConditionalsOperators
    #[token("==")]
    Equality,
    #[token("===")]
    TrueEquality,
    #[token("!=")]
    NotEqual,
    #[token(">")]
    Greater,
    #[token("<")]
    Less,
    #[token(">=")]
    GreaterOEqual,
    #[token("<=")]
    LessOEqual,

    //MISCSymbols
    #[token(">>")]
    AppendW,
    #[token("<<")]
    AppendR,
    #[token("->")]
    Print,

    // IDENTIFIERS
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", priority = 1)]
    Name,

    // KEYWORDS
    #[regex(r"if|elif|else|while|unless|for|until|switch")]
    ControlBlock,
    #[regex(r"print")]
    BuiltIn,
    #[regex(r"\$|@|%|")]
    DataType,

    // LITERALS (simplified)
    #[regex(r"\d+", priority = 2)]
    Number,
    #[regex(r#"["'][^"']*["']"#)]
    String,
    #[regex(r"[Tt][Rr][Uu][Ee]|[Ff][Aa][Ll][Ss][Ee]|1|0", priority = 3)]
    Bool,
}

pub fn openFile(fileName: &str) -> String {
    // Try to open the file
    let mut file: File = File::open(fileName).expect("Failed to open file");

    // Make a string to hold the contents
    let mut contents: String = String::new();

    // Read the file into the string
    file.read_to_string(&mut contents).expect("Failed to read file");

    // Return the string containing the code from the Jade File
    return contents;
}

/// turns the raw code into a list of token:value pairs.
/// Used after openFile()
pub fn lexCode(rawCode: String) -> Vec<(Token, String)> {
    let mut tokens: Vec<(Token, String)> = Vec::new();

    let mut lexer = Token::lexer(rawCode.as_str());

    while let Some(token) = lexer.next() {
        // `lexer.slice()` returns the actual text that matched the token
        //println!("Token: {:?} => Value: {:?}", token, lexer.slice());
        tokens.push((token.unwrap(), lexer.slice().to_string()));
    }

    return tokens;
}
