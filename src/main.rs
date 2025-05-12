#![allow(non_snake_case)]
//#![allow(unused_variables)]
#![allow(unused_parens)]
#![allow(unused_mut)]


// Import the packages
mod lexer;
mod parser;
mod evaluator;
mod runtime;
mod jadeErrors;
mod jadeTypes {
    pub mod primitives;
}

// Import functions from lexer
use lexer::openFile;
use lexer::lexCode;
use crate::lexer::Token;

// Import functions from parser
use parser::chunkCode;
use parser::parseCode;
use parser::TokenBlock;
use parser::ParsedBlock;

// Import functions from runtime
use runtime::jruntime;

/*
    AUTHOR:         Dyl C.
    DATE:           5/5/2025
    PROJECT NAME:   Jade Programming Language
    OTHER CONTRIBUTERS: 
                    Jman:
                        First Tester and go-to person for asking
                        "How does this look?"
                            
*/

fn main() {

    // The Jade code extracted from the file. 
    let rawCode: String = openFile("firstTest.bg");
    
    // The tokens extracted from the raw Code
    let tokens: Vec<(Token, String)> = lexCode(rawCode);
   
    // Chunk the tokens into blocks that will be read by the parser
    let tokenBlocks: Vec<TokenBlock> = chunkCode(tokens);
    // Take the blocks and turn them into structs of data
    let parsedCode: Vec<ParsedBlock> = parseCode(tokenBlocks);
    // At this point we'll enter runtime.
    jruntime(parsedCode);
    // At this point the code will run and do its thing.

    // Last Step: Win. Hopefully.

}

