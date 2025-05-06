#![allow(non_snake_case)]

mod lexer;
mod parser;
mod runtime;
mod jadeTypes {
    pub mod primitives;
}

// Import functions from lexer
use lexer::openFile;

// use regex::Regex;


fn main() {
    /*
        Main function. Will call the other functions in the program.

        Functions Called:
            openFile    - opens the Jade file and reads it. Returns a 
                list containing the lines. Calls scanCode wich chunks 
                the code into more manageable blocks for the parser.
    */
    
    // Scan the code and chunk it
    let blocks:Vec<String> = openFile("src/firstTest.bg");
    println!("{:?}", blocks,);
}
