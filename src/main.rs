#![allow(non_snake_case)]
//#![allow(unused_variables)]
#![allow(unused_parens)]
#![allow(unused_mut)]


// Import the packages
mod lexer;
mod parser;
mod runtime;
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
use std::collections::HashMap;

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
    /*
        Main function. Will call the other functions in the program.

        Functions Called:
            openFile    - opens the Jade file and reads it. Returns a 
                list containing the lines. 
                
            lexCode     - Turns the raw code into token pairs that will
                be used by the parser.

            chunkCode   - Turns the code into more readable chunks for
                the parser.

            parseBlocks - will be used with a for-loop to iterate
                through the blocks of code and turn them into actual
                readable data for Jade.
    */

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

