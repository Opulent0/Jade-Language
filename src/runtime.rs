use crate::evaluator::evaluate;
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
        println!("Declaring {} as {:?}", name, value);
        self.variables.insert(name, value);
    }

    // Get the value of a variable
    pub fn get_variable(&self, name: &str) -> Option<&Value> {
        self.variables.get(name)
    }

    // Set the value of an existing variable
    pub fn set_variable(&mut self, name: String, value: Value) {
        if let Some(var) = self.variables.get_mut(&name) {
            println!("Setting {} as {:?}", name, value);
            *var = value;
        }
    }
}

pub fn jruntime(code: Vec<ParsedBlock>) {
    let mut variables = Runtime::new();
    for block in code {
        match block.blockType.as_str() {
            "VarDec" => {
                let expr = block.value.unwrap();
                let expected_type = block.datatype.clone(); // e.g., Some("int8")
                let value = evaluate(&expr, &variables, expected_type.as_deref());
                variables.declare_variable(block.name.unwrap(), value);
            },
            
            "VarSet" => {
                // Set an existing variable to a new value
                if let Some(ref name) = block.name {
                    //println!("Parsed block value: {:?}", block.value);
                    if let Some(expr) = block.value {
                        let existing = variables.get_variable(name).unwrap();
                        let expected_type = match existing {
                            Value::Int8(_) => Some("int8"),
                            Value::UInt8(_) => Some("uint8"),
                            Value::Int16(_) => Some("int16"),
                            Value::UInt16(_) => Some("uint16"),
                            Value::Int32(_) => Some("int32"),
                            Value::UInt32(_) => Some("uint32"),
                            Value::Int64(_) => Some("int64"),
                            Value::UInt64(_) => Some("uint64"),
                            Value::Float32(_) => Some("float32"),
                            Value::Float64(_) => Some("float64"),
                            _ => None
                        };
                        let new_value = evaluate(&expr, &variables, expected_type);
                        variables.set_variable(name.clone(), new_value);
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

fn printStatement(printVar: &Value) {
    println!("{:?}", printVar)
}