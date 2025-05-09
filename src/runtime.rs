
use std::collections::HashMap;
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
    pub variables: HashMap<String, Value>,
}
#[allow(dead_code)]
impl Runtime {
    pub fn new() -> Self {
        Runtime {
            variables: HashMap::new(),
        }
    }

    // Add a new variable
    pub fn declare_variable(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }

    // Get the value of a variable
    pub fn get_variable(&self, name: &str) -> Option<&Value> {
        self.variables.get(name)
    }

    // Set the value of an existing variable
    pub fn set_variable(&mut self, name: String, value: Value) {
        if let Some(var) = self.variables.get_mut(&name) {
            *var = value;
        }
    }
}

pub fn jruntime(code: Vec<ParsedBlock>) {
    let mut variables = Runtime::new();
    for block in code {
        match block.blockType.as_str() {
            "VarDec" => {
                let dtype = getDataType(block.datatype.unwrap(), block.value.unwrap());
                // Variable is already stored from the parser, no need to do anything here
                println!("Declaring Variable: {:?}", block.name.clone().unwrap());
                variables.declare_variable(block.name.unwrap(), dtype.clone());
            },
            
            "VarSet" => {
                // Set an existing variable to a new value
                if let Some(ref name) = block.name {
                    let dtype = variables.get_variable(name);
                    if let mut datatype =  dtype {
                        let new_value = dtype.clone();
                        // Update the value of the variable
                        datatype = new_value.clone();
                        println!("Updated Variable {} to {:?}", name, new_value.unwrap());
                    } else {
                        println!("Error: Variable '{}' not found", name);
                    }
                }
            },
            
            "PrintVar" => {
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

/// Takes the String and returns the datatype of the variable
/// This is a placeholder function and will be replaced with
fn getDataType(typing: String, value: String) -> Value {
    match typing.as_str() {
        "int8" => Value::Int8(value.parse::<i8>().unwrap_or(0)),
        "uint8" => Value::UInt8(value.parse::<u8>().unwrap_or(0)),
        "int16" => Value::Int16(value.parse::<i16>().unwrap_or(0)),
        "uint16" => Value::UInt16(value.parse::<u16>().unwrap_or(0)),
        "int32" => Value::Int32(value.parse::<i32>().unwrap_or(0)),
        "uint32" => Value::UInt32(value.parse::<u32>().unwrap_or(0)),
        "int64" => Value::Int64(value.parse::<i64>().unwrap_or(0)),
        "uint64" => Value::UInt64(value.parse::<u64>().unwrap_or(0)),
        "float32" => Value::Float32(value.parse::<f32>().unwrap_or(0.0)),
        "float64" => Value::Float64(value.parse::<f64>().unwrap_or(0.0)),
        "string" => Value::String(value.clone()),
        "bool" => Value::Bool(value.parse::<bool>().unwrap_or(false)),

        "int" => {
            if let Ok(v) = value.parse::<i8>() {
                Value::Int8(v)
            } else if let Ok(v) = value.parse::<u8>() {
                Value::UInt8(v)
            } else if let Ok(v) = value.parse::<i16>() {
                Value::Int16(v)
            } else if let Ok(v) = value.parse::<u16>() {
                Value::UInt16(v)
            } else if let Ok(v) = value.parse::<i32>() {
                Value::Int32(v)
            } else if let Ok(v) = value.parse::<u32>() {
                Value::UInt32(v)
            } else if let Ok(v) = value.parse::<i64>() {
                Value::Int64(v)
            } else if let Ok(v) = value.parse::<u64>(){
                Value::UInt64(v)
            } else {
                Value::Null // Fallback to null if parsing fails
            }
        },

        "float" => {
            if let Ok(v) = value.parse::<f32>() {
                Value::Float32(v)
            } else if let Ok(v) = value.parse::<f64>() {
                Value::Float64(v)
            } else {
                Value::Null // Fallback to null if parsing fails
            }
        },

        "$" => {
            // Infer type from value string
            if let Ok(v) = value.parse::<i64>() {
                getDataType("int".to_string(), v.to_string())
            } else if let Ok(v) = value.parse::<f64>() {
                getDataType("float".to_string(), v.to_string())
            } else if value == "TRUE" || value == "FALSE" {
                Value::Bool(value == "true")
            } else {
                // Fallback to string
                Value::String(value)
            }
        },

        _ => Value::Null, // Unknown type
    }
}

fn printStatement(printVar: &Value) {
    match printVar {
        Value::Int8(v) => println!("{}", v),
        Value::UInt8(v) => println!("{}", v),
        Value::Int16(v) => println!("{}", v),
        Value::UInt16(v) => println!("{}", v),
        Value::Int32(v) => println!("{}", v),
        Value::UInt32(v) => println!("{}", v),
        Value::Int64(v) => println!("{}", v),
        Value::UInt64(v) => println!("{}", v),
        Value::String(v) => println!("{}", v),
        Value::Bool(v) => println!("{}", v),
        Value::Float32(v) => println!("{}", v),
        // Add other cases as needed
        other => println!("Unhandled value: {:?}", other),
    }
}