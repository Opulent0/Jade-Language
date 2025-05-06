pub fn unknownSyntaxError(line:&str) {
    panic!("Error! unkown line! {line} {}", "\n\nDyl, Remember to add more to this error so it actually gives useful info!!!");
}

pub fn noEndingSemiColon(line:&str) {
    panic!("Error! line needs semicolon! {line}");
}