use crate::runtime::Value;

/*

pub fn unknownSyntaxError(line:&str) {
    panic!("Error! unkown line! {line} {}", "\n\nDyl, Remember to add more to this error so it actually gives useful info!!!");
}

pub fn noEndingSemiColon(line:&str) {
    panic!("Error! line needs semicolon! {line}");
}

*/

pub fn parsingError() {
    eprintln!("WTF is this dude!!? You never told me this was an option!?");
}

pub fn valueError(type_str: &str, other: Value) {
    println!("\n\n____________________________________________\nVALUE ERROR:");
    panic!("Error! Type mismatch! Expected: {type_str}, Found: {:?}\n____________________________________________", other);
}

pub fn variableNotFoundError(name: &str) {
    eprintln!("Variable '{}' not found", name);
}