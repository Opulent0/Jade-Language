use crate::jadeErrors::parsingError;
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
    pub value: Option<Expression>,
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
        let tokens: &Vec<(Token, String)> = &block.tokens;

        if tokens.is_empty() {
            continue;
        }

        match tokens[0].0 {
            Token::Var | Token::Const | Token::Sink => {
                // Handle variable declaration as before
                let identifier: String = tokens.get(0).map(|(_, v)| v.clone()).unwrap_or_default();
                let name: Option<String> = tokens.get(1).map(|(_, v)| v.clone());
                let datatype: Option<String> = Some(tokens.get(2).map(|(_, v)| v.clone()).unwrap());

                // Length of the expression/value we are setting the
                // variable to
                let length: usize = tokens.len();

                // Vec containing the expression
                let newExpression = tokens[4..length].to_vec();

                parsedCode.push(ParsedBlock {
                    blockType: String::from("VarDec"),
                    identifier,
                    name,
                    datatype,
                    value: Some(evaluateExpression(newExpression)),
                    ..Default::default()
                });
            }

            Token::Name => {
                // Variable assignment
                if (tokens.get(1).map(|(v, _)| v).unwrap() == &(Token::Equals)) {
                    // Handle variable assignment (setting new value)
                    // Get the variable name
                    let varName = tokens.get(0).map(|(_, v)| v.clone()).unwrap_or_default();

                    // Length of the expression/value we are setting the
                    // variable to
                    let length: usize = tokens.len();

                    // Vec containing the expression
                    let newExpression = tokens[2..length].to_vec();
        
                        parsedCode.push(ParsedBlock {
                        blockType: String::from("VarSet"),
                        name: Some(varName),
                        value: Some(evaluateExpression(newExpression)),
                        ..Default::default()
                    });
                    
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
            Token::Division => {
                if tokens.get(1).map(|(v, _)| v.clone()) == Some(Token::Division) {

                }
            }
            _ => {
                println!("Unrecognized block: {:?}", tokens);
            }
        }
    }

    parsedCode
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
    Exponent,
    Root,
    Equality,
    TrueEquality,
    Modulo,
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterOrEqual,
    LessOrEqual,
    Not,
    Xor,
    Nor,
    Nand,
    Xnor,
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

fn evaluateExpression (expressionTokens: Vec<(Token, String)>) -> Expression {
    let mut expression: Expression;
    let expLen: usize = expressionTokens.len();

    // We either use the value given (if it's a single token) or
    // we create a binary-op
    if expLen == 1 {
        expression = evalToken(expressionTokens.get(0).unwrap().clone());
    } else {
        let mut op: BinaryOperator = BinaryOperator::Not;
        let mut opIdx: usize = 0;
        let mut parenDepth: u8 = 0;
        for (idx, token) in expressionTokens.iter().enumerate() {
            //println!("Token: {:?}\tValue: {:?}\nIndex: {:?}\tParenDepth: {:?}\n", token.0, token.1, idx, parenDepth);
            
            match token.0 {
                Token::OpenParen  => parenDepth += 1,
                Token::CloseParen => parenDepth -= 1,
                Token::Addition|Token::Subtraction|Token::Division|Token::Multiplication|
                Token::Greater|Token::GreaterOEqual|Token::Less|Token::LessOEqual|Token::Equality|
                Token::Exponent|Token::NotEqual|Token::TrueEquality|Token::Root
                => {
                    if parenDepth == 0 {
                        op = match token.0 {
                            Token::Addition => BinaryOperator::Add,
                            Token::Subtraction => BinaryOperator::Subtract,
                            Token::Division => BinaryOperator::Divide,
                            Token::Multiplication => BinaryOperator::Multiply,
                            Token::Greater => BinaryOperator::GreaterThan,
                            Token::GreaterOEqual => BinaryOperator::GreaterOrEqual,
                            Token::Less => BinaryOperator::LessThan,
                            Token::LessOEqual => BinaryOperator::LessOrEqual,
                            Token::Equality => BinaryOperator::Equal,
                            Token::Exponent => BinaryOperator::Exponent,
                            Token::Modulo => BinaryOperator::Modulo,
                            Token::And => BinaryOperator::And,
                            Token::Or => BinaryOperator::Or,
                            Token::Not => BinaryOperator::Not,
                            Token::NotEqual => BinaryOperator::NotEqual,
                            Token::TrueEquality => BinaryOperator::TrueEquality,
                            Token::Root => BinaryOperator::Root,
                            Token::Xor => BinaryOperator::Xor,
                            Token::Nor => BinaryOperator::Nor,
                            Token::Nand => BinaryOperator::Nand,
                            Token::Xnor => BinaryOperator::Xnor,
                            _ => {
                                parsingError();
                                BinaryOperator::Or}
                        };
                        opIdx = idx;
                    }
                }

                _=> {}
            }
        }
        // Set the left and right expressions based on the position
        // of the operator
        let mut left = expressionTokens[0..opIdx].to_vec();
        if left.get(0) == Some(&(Token::OpenParen, String::from("("))) {
            left.remove(0);
            left.remove(left.len()-1);
        }
        

        let mut right = expressionTokens[opIdx+1..expLen].to_vec();
        if right.get(0) == Some(&(Token::OpenParen, String::from("("))) {
            right.remove(0);
            right.remove(right.len()-1);
        }

        let leftExpression = evaluateExpression(left);
        let rightExpression = evaluateExpression(right);
        
        
        expression = Expression::BinaryOp {
            op: op,
            left: Box::new(leftExpression),
            right: Box::new(rightExpression),
        };
    }
    expression
}

fn evalToken(token: (Token, String)) -> Expression {
    match token.0 {
        Token::Name => Expression::Variable(token.1),
        Token::String => Expression::String(token.1),
        Token::Number => {
            let parsed = token.1.parse::<f64>().unwrap_or(0.0);
            Expression::Number(parsed)
        }
        Token::Bool => {
            let val = token.1.to_lowercase();
            Expression::Boolean(val == "true" || val == "1")
        }
        _ => {
            parsingError();
            Expression::Number(0.0)
        }
    }
}
