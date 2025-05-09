use crate::lexer::Token;


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

/// A struct for the binary operators. This will be used to parse the
/// binary operators in the code.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterOrEqual,
    LessOrEqual,
    And,
    Or,
}


/// A struct for the binary operators. This will be used to parse the
/// binary operators in the code.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Number(f64),
    String(String),
    Boolean(bool),
    Variable(String),
    BinaryOp {
        op: BinaryOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    // Add more as needed: UnaryOp, Call, Index, etc.
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
            "{" => {braceDepth += 1;},
            
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
#[allow(dead_code)]
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
                // Handle variable declaration as before
                let identifier: String = tokens.get(0).map(|(_, v)| v.clone()).unwrap_or_default();
                let name: Option<String> = tokens.get(1).map(|(_, v)| v.clone());
                let datatype: Option<String> = Some(tokens.get(2).map(|(_, v)| v.clone()).unwrap());

                let value = tokens.iter().position(|(tok, _)| *tok == Token::Equals)
                    .and_then(|eq_index| tokens.get(eq_index + 1).map(|(_, v)| v.clone()));

                parsedCode.push(ParsedBlock {
                    blockType: String::from("VarDec"),
                    identifier,
                    name,
                    datatype,
                    value,
                    ..Default::default()
                });
            }

            Token::Name => {
                // Variable assignment
                if (tokens.get(1).map(|(v, _)| v).unwrap() == &(Token::Equals)) {
                    // Handle variable assignment (setting new value)
                    let varName = tokens.get(0).map(|(_, v)| v.clone()).unwrap_or_default();
                    let newValue = tokens.get(2).map(|(_, v)| v.clone());
                    
                    parsedCode.push(ParsedBlock {
                        blockType: String::from("VarSet"),
                        name: Some(varName),
                        value: newValue,
                        ..Default::default()
                    })
                // Print using "->" operator
                } else if (tokens.get(1).map(|(v, _)| v).unwrap() == &(Token::Print)) {
                    let varName= tokens.get(0).map(|(_, v)| v.clone()).unwrap_or_default();
                    
                    parsedCode.push(ParsedBlock {
                        blockType: String::from("PrintVar"),
                        name: Some(varName),
                        ..Default::default()
                    })
                }
            }
            _ => {
                println!("Unrecognized block: {:?}", tokens);
            }
        }
    }

    parsedCode
}

