#![allow(non_snake_case)]

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

/*
    AUTHOR:         Dyl C.
    DATE:           5/5/2025
    PROJECT NAME:   Jade Programming Language
    OTHER CONTRIBUTERS: 
                    Jman:
                        First Tester and go-to person for asking
                        "How does this look?"
                            
*/


/// A Struct for each different possible property of a parsed block.
#[derive(Debug, Default)]
pub struct ParsedBlock {
    pub blockType: String,
    pub identifier: String,
    pub name: Option<String>,
    pub datatype: Option<String>,
    pub value: Option<String>,
    pub condition: Option<String>,
    pub body: Option<Vec<ParsedBlock>>, // Nested blocks after parsing
    pub parameters: Option<Vec<String>>,
    pub returnType: Option<String>,
}


fn main() {
    /*
        Main function. Will call the other functions in the program.

        Functions Called:
            openFile    - opens the Jade file and reads it. Returns a 
                list containing the lines. 
                
            lexCode     - Turns the raw code into token pairs that will
                be used by the parser.

            parseBlocks - will be used with a for-loop to iterate
                through the blocks of code and turn them into actual
                readable data for Jade.
    */

    // The Jade code extracted from the file. 
    let rawCode: String = openFile("firstTest.bg");
    
    // The tokens extracted from the raw Code
    let tokens: Vec<(Token, String)> = lexCode(rawCode);
    print!("Tokens: \n{:?}", tokens);

    // Chunk the tokens into blocks that will be read by the parser
    let tokenBlocks = chunkCode(tokens);

    // Take the blocks and turn them into structs of data
    let parsedCode = parseCode(tokenBlocks);

    // At this point we'll enter runtime.

    // At this point the code will run and do its thing.

    // Last Step: Win. Hopefully.

}

#[derive(Debug)]
pub struct TokenBlock {
    pub tokens: Vec<(Token, String)>,
}

pub fn chunkCode (tokens: Vec<(Token,String)>) -> Vec<TokenBlock> {
    let mut tokenBlocks: Vec<TokenBlock> = Vec::new();
    // Add logic here
    return tokenBlocks;
}

pub fn parseCode(tokenBlocks: Vec<TokenBlock>) -> Vec<ParsedBlock> {
    let mut parsedCode: Vec<ParsedBlock> = Vec::new();
    // Add logic here
    return parsedCode;
}