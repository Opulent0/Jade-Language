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
    print!("\n{:?}", parsedCode);
    // At this point we'll enter runtime.

    // At this point the code will run and do its thing.

    // Last Step: Win. Hopefully.

}

/// A block of tokens. Will only contain the tokens pertaining to the
/// block of block.
#[derive(Debug)]
pub struct TokenBlock {
    pub tokens: Vec<(Token, String)>,
}

// Add some functionality to the TokenBlock Struct
impl TokenBlock {
    // Function that needs a mutable reference to self
    fn addItem(&mut self, item: (Token, String)) {
        self.tokens.push(item);
    }
}

/// Chunks the code into pieces to be read by the parsCode fn.
pub fn chunkCode (tokens: Vec<(Token,String)>) -> Vec<TokenBlock> {
    // Setup variables we will use later for creating the parsed data.+
    let mut tokenBlocks: Vec<TokenBlock> = Vec::new();
    let mut currentBlock = TokenBlock { tokens: Vec::new() };

    let mut braceDepth: u8 = 0;
    
    // This shit is probably confusing so lemme break it down
    // We loop through token:value pairs and do a case statement
    // We increment or decrement braceDepth based on the token type
    // We also check for semicolons to determine if the line should
    // end the block.
    for t in tokens {
        match t.1.as_str() {
            
            // increment braceDepth to enter a block of code
            "{" => {braceDepth += 1},
            
            // decrement braceDepth to exit the Layer of code0
            "}" => {
                braceDepth -= 1;
                if braceDepth == 0 {
                    tokenBlocks.push(currentBlock);
                    currentBlock = TokenBlock { tokens: Vec::new() }; // fresh block
                }},
            ";" => {if (braceDepth == 0) {
                tokenBlocks.push(currentBlock);
                currentBlock = TokenBlock { tokens: Vec::new() }; // fresh block
            }},
            _   => currentBlock.addItem(t)
        }
    }

    return tokenBlocks;
}

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

/// Takes the chunked code and breaks it down even further.
/// It only takes what is nessesary and returns a Vec of structs. 
pub fn parseCode(tokenBlocks: Vec<TokenBlock>) -> Vec<ParsedBlock> {
    let mut parsedCode: Vec<ParsedBlock> = Vec::new();

    for block in tokenBlocks {
        let tokens = &block.tokens;

        if tokens.is_empty() {
            continue;
        }

        match tokens[0].0 {
            Token::Var | Token::Const | Token::Sink => {
                let identifier = tokens.get(1).map(|(_, v)| v.clone()).unwrap_or_default();
                let value = tokens.get(3).map(|(_, v)| v.clone());

                parsedCode.push(ParsedBlock {
                    blockType: String::from("VariableDecl"),
                    identifier,
                    value,
                    ..Default::default()
                });
            }

            Token::Sink => {
                // Example sink statement
                parsedCode.push(ParsedBlock {
                    blockType: String::from("SinkStatement"),
                    identifier: tokens.get(1).map(|(_, v)| v.clone()).unwrap_or_default(),
                    ..Default::default()
                });
            }

            _ => {
                println!("Unrecognized token sequence: {:?}", tokens);
            }
        }
    }

    return parsedCode;
}