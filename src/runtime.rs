use crate::evaluator::evaluate;
use std::{collections::HashMap};
use crate::parser::ParsedBlock;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Value {
    Int8(i8),
    UInt8(u8),
    Int16(i16),
    UInt16(u16),
    Int32(i32),
    UInt32(u32),
    Int64(i64),
    UInt64(u64),

    // Floating-point
    Float(f64),       // General-purpose float
    Float32(f32),
    Float64(f64),

    // Other primitives
    Bool(bool),
    String(String),

    // Composite types
    Array(Vec<Value>),                  // Flexible runtime-length array
    FixedArray(Vec<Value>, usize),      // For fixed-length arrays
    Slice(Box<[Value]>),                // Slice reference-like
    Pointer(Box<Value>),                // Simulated pointer

    // Optional / Null
    Null,
}

#[derive(Default)]
pub struct Runtime {
    // A hashmap to store variables and their values
    // The key is the variable name, and the value is a Value enum
    // that can hold different types of data.
    pub variables: HashMap<String, Box<(Value, String)>>,
}
#[allow(dead_code)]
impl Runtime {
    pub fn new() -> Self {
        Runtime {
            variables: HashMap::new(),
        }
    }

    // Add a new variable
    pub fn declare_variable(&mut self, name: String, value: Value, actualType: String) {
        println!("Declaring {} as {:?}", name, value);
        self.variables.insert(name, Box::new((value, actualType)));
    }

    // Get the value of a variable
    pub fn get_variable(&self, name: &str) -> Option<&Value> {
        Some(&self.variables.get(name).unwrap().0)
    }

    // Set the value of an existing variable
    pub fn set_variable(&mut self, name: String, value: Value) {
        if let Some(var) = self.variables.get_mut(&name) {
            println!("Setting {} as {:?}", name, value);
            var.0 = value;
        }
    }

    pub fn getVarType (&self, name: &str) -> &String {
        let varType = &self.variables.get(name).unwrap().1;
        return varType;
    }
}

pub fn jruntime(code: Vec<ParsedBlock>) {
    let mut variables = Runtime::new();
    for block in code {
        match block.blockType.as_str() {
            "VarDec"    => {
                variables.declare_variable(
                    block.name.unwrap().to_string(),
                    evaluate(&block.value.unwrap(), &variables, Some(block.datatype.clone().unwrap())),
                    block.datatype.unwrap()
                );
            }

            "VarSet"    => {
                variables.set_variable(block.name.clone().unwrap(), evaluate(&block.value.unwrap(), &variables, Some(variables.getVarType(block.name.unwrap().as_str()).clone())));
            }
            
            "PrintVar"  => {
                // Print the value of a variable
                let dtype: Option<&Value> = variables.get_variable(block.name.unwrap().as_str());
                let printVar: &Value = dtype.clone().unwrap();

                // Print the variable value
                printStatement(printVar);
            }
            _ => {
                // Handle other types of blocks, like control flow
            }
        }
    }
}

fn printStatement(printVar: &Value) {

    println!("{:?}", printVar)
}