use crate::parser::{Expression, BinaryOperator};
use crate::runtime::{Runtime, Value};
use crate::jadeErrors::{valueError, variableNotFoundError};

#[allow(dead_code, unreachable_patterns)]
pub fn evaluate(expr: &Expression, runtime: &Runtime, expected_type: Option<String> ) -> Value {
    
    let val = match expr {
        Expression::String(val) => Value::String(val.clone()),
        
        Expression::Float(val) => {
            val.to_string().parse::<f64>().map(Value::Float64).unwrap_or(Value::Null)
        }

        Expression::Integer(val) => {
            val.to_string().parse::<i64>().map(Value::Int64).unwrap_or(Value::Null)
        }

        Expression::Boolean(val) => Value::Bool(*val),

        Expression::Variable(name) => {
            let varVal = runtime.get_variable(name).cloned().unwrap_or({variableNotFoundError(name); Value::Null});
            return varVal
        }

        Expression::BinaryOp { op, left, right } => {
            let lval = evaluate(left, runtime, None);
            let rval = evaluate(right, runtime, None);
            evaluate_binary_op(op, lval, rval)
        }

        _ => {
            eprintln!("Unsupported expression: {:?}", expr);
            Value::Null
        }
    };

    if let Some(ref t) = expected_type {
        coerce_to_type(val, t)
    } else {
        val
    }
}

fn coerce_to_type(val: Value, type_str: &str) -> Value {
    use Value::*;
    match (type_str, val) {
        
        // SCALAR Values
        ("$", Int64(v)) => {
            let returnVal = if -128 <= v && v <= 127 {
                Int8(v as i8)
            } else if v >= 0 && v <= 255 {
                UInt8(v as u8)
            } else if -32768 <= v && v <= 32767 {
                Int16(v as i16)
            } else if v >= 0 && v <= 65535 {
                UInt16(v as u16)
            } else if -2147483648 <= v && v <= 2147483647 {
                Int32(v as i32)
            } else if v >= 0 && v <= 4294967 {
                UInt32(v as u32)
            } else if -9223372036854775808 <= v && v <= -9223372036854775807 {
                Int64(v as i64)
            } else {
                UInt64(v as u64)
            };

            returnVal
        },

        ("$", UInt64(v)) => {
            let returnVal = if v <= 255 {
                UInt8(v as u8)
            } else if v <= 65535 {
                UInt16(v as u16)
            } else if v <= 4294967 {
                UInt32(v as u32)
            } else {
                UInt64(v as u64)
            };

            returnVal
        },

        ("$", Float64(v)) => Float64(v as f64),
        ("$", Bool(v)) => Bool(v as bool),
        ("$", String(v)) => String(v as std::string::String),

        ("int", Int8(v)) => {
            let returnVal = Int8(v as i8);
            returnVal
        },

        ("int", UInt8(v)) => {
            let returnVal = if v <= 127 {
                Int8(v as i8)
            } else {
                UInt8(v as u8)
            };

                returnVal
        },        

        ("int", Int16(v)) => {
            let returnVal = if -128 <= v && v <= 127 {
                    Int8(v as i8)
                } else if v >= 0 && v <= 255 {
                    UInt8(v as u8)
                } else {
                    Int16(v as i16)
                };

                returnVal
        },

        ("int", UInt16(v)) => {
            let returnVal = if v <= 127 {
                Int8(v as i8)
            } else if v <= 255 {
                UInt8(v as u8)
            } else if v <= 32767 {
                Int16(v as i16)
            } else {
                UInt16(v as u16)
            };

                returnVal
        },

        ("int", Int32(v)) => {
        let returnVal = if -128 <= v && v <= 127 {
                Int8(v as i8)
            } else if v >= 0 && v <= 255 {
                UInt8(v as u8)
            } else if -32768 <= v && v <= 32767 {
                Int16(v as i16)
            } else if v >= 0 && v <= 65535 {
                UInt16(v as u16)
            } else {
                Int32(v as i32)
            };

            returnVal
        },

        ("int", UInt32(v)) => {
            let returnVal = if v <= 127 {
                Int8(v as i8)
            } else if v <= 255 {
                UInt8(v as u8)
            } else if v <= 32767 {
                Int16(v as i16)
            } else if v <= 65535 {
                UInt16(v as u16)
            } else if v <= 2147483647 {
                Int32(v as i32)
            } else {
                UInt32(v as u32)
            };

                returnVal
        },

        // INT Types
        ("int", Int64(v)) => {
        let returnVal = if -128 <= v && v <= 127 {
                Int8(v as i8)
            } else if v >= 0 && v <= 255 {
                UInt8(v as u8)
            } else if -32768 <= v && v <= 32767 {
                Int16(v as i16)
            } else if v >= 0 && v <= 65535 {
                UInt16(v as u16)
            } else if -2147483648 <= v && v <= 2147483647 {
                Int32(v as i32)
            } else if v >= 0 && v <= 4294967 {
                UInt32(v as u32)
            } else {
                Int64(v as i64)
            };

            returnVal
        },

        ("int", UInt64(v)) => {
            let returnVal = if v <= 127 {
                Int8(v as i8)
            } else if v <= 255 {
                UInt8(v as u8)
            } else if v <= 32767 {
                Int16(v as i16)
            } else if v <= 65535 {
                UInt16(v as u16)
            } else if v <= 2147483647 {
                Int32(v as i32)
            } else if v <= 4294967 {
                UInt32(v as u32)
            } else if v <= 9223372036854775807 {
                Int64(v as i64)
            } else {
                UInt64(v as u64)
            };

                returnVal
        },

        ("int8", Int64(v))     => Int8(v as i8),
        ("uint8", Int64(v))    => UInt8(v as u8),
        ("int16", Int64(v))    => Int16(v as i16),
        ("uint16", Int64(v))   => UInt16(v as u16),
        ("int32", Int64(v))    => Int32(v as i32),
        ("uint32", Int64(v))   => UInt32(v as u32),
        ("uint64", Int64(v))   => UInt64(v as u64),

        ("int8", Int8(v))     => Int8(v as i8),
        ("uint8", UInt8(v))    => UInt8(v as u8),
        ("int16", Int16(v))    => Int16(v as i16),
        ("uint16", UInt16(v))   => UInt16(v as u16),
        ("int32", Int32(v))    => Int32(v as i32),
        ("uint32", UInt32(v))   => UInt32(v as u32),
        ("int64", Int64(v))    => Int64(v as i64),
        ("uint64", UInt64(v))   => UInt64(v as u64),

        (_, other) => {
            valueError(type_str, other);
            Null
        }
    }
}

fn evaluate_binary_op(op: &BinaryOperator, l: Value, r: Value) -> Value {
    use Value::*;
    match op {
        BinaryOperator::Add => match (l.clone(), r.clone()) {
            // Int8
            (Int8(a), Int8(b))          => Int8(a + b),
            (Int8(a), UInt8(b))         => Int8(a + b as i8),
            (Int8(a), Int16(b))        => Int8(a + b as i8),
            (Int8(a), UInt16(b))       => Int8(a + b as i8),
            (Int8(a), Int32(b))        => Int8(a + b as i8),
            (Int8(a), UInt32(b))       => Int8(a + b as i8),
            (Int8(a), Int64(b))        => Int8(a + b as i8),
            (Int8(a), UInt64(b))       => Int8(a + b as i8),
            (Int8(a), Float32(b))      => Int8(a + b.round() as i8),
            (Int8(a), Float64(b))      => Int8(a + b.round() as i8),

            // Uint8
            (UInt8(a), UInt8(b))        => UInt8(a + b),
            (UInt8(a), Int8(b))         => UInt8(a + b as u8),
            (UInt8(a), Int16(b))       => UInt8(a + b as u8),
            (UInt8(a), UInt16(b))      => UInt8(a + b as u8),
            (UInt8(a), Int32(b))       => UInt8(a + b as u8),
            (UInt8(a), UInt32(b))      => UInt8(a + b as u8),
            (UInt8(a), Int64(b))       => UInt8(a + b as u8),
            (UInt8(a), UInt64(b))      => UInt8(a + b as u8),
            (UInt8(a), Float32(b))     => UInt8(a + b.round() as u8),
            (UInt8(a), Float64(b))     => UInt8(a + b.round() as u8),

            // Int16
            (Int16(a), Int16(b))      => Int16(a + b),
            (Int16(a), Int8(b))        => Int16(a + b as i16),
            (Int16(a), UInt8(b))       => Int16(a + b as i16),
            (Int16(a), UInt16(b))     => Int16(a + b as i16),
            (Int16(a), Int32(b))      => Int16(a + b as i16),
            (Int16(a), UInt32(b))     => Int16(a + b as i16),
            (Int16(a), Int64(b))      => Int16(a + b as i16),
            (Int16(a), UInt64(b))     => Int16(a + b as i16),
            (Int16(a), Float32(b))    => Int16(a + b.round() as i16),
            (Int16(a), Float64(b))    => Int16(a + b.round() as i16),

            // UInt16
            (UInt16(a), UInt16(b))    => UInt16(a + b),
            (UInt16(a), Int8(b))       => UInt16(a + b as u16),
            (UInt16(a), UInt8(b))      => UInt16(a + b as u16),
            (UInt16(a), Int16(b))     => UInt16(a + b as u16),
            (UInt16(a), Int32(b))     => UInt16(a + b as u16),
            (UInt16(a), UInt32(b))    => UInt16(a + b as u16),
            (UInt16(a), Int64(b))     => UInt16(a + b as u16),
            (UInt16(a), UInt64(b))    => UInt16(a + b as u16),
            (UInt16(a), Float32(b))   => UInt16(a + b.round() as u16),
            (UInt16(a), Float64(b))   => UInt16(a + b.round() as u16),

            // Int32
            (Int32(a), Int32(b))      => Int32(a + b),
            (Int32(a), Int8(b))        => Int32(a + b as i32),
            (Int32(a), UInt8(b))       => Int32(a + b as i32),
            (Int32(a), Int16(b))      => Int32(a + b as i32),
            (Int32(a), UInt16(b))     => Int32(a + b as i32),
            (Int32(a), UInt32(b))     => Int32(a + b as i32),
            (Int32(a), Int64(b))      => Int32(a + b as i32),
            (Int32(a), UInt64(b))     => Int32(a + b as i32),
            (Int32(a), Float32(b))    => Int32(a + b.round() as i32),
            (Int32(a), Float64(b))    => Int32(a + b.round() as i32),

            // UInt32
            (UInt32(a), UInt32(b))    => UInt32(a + b),
            (UInt32(a), Int8(b))       => UInt32(a + b as u32),
            (UInt32(a), UInt8(b))      => UInt32(a + b as u32),
            (UInt32(a), Int16(b))     => UInt32(a + b as u32),
            (UInt32(a), UInt16(b))    => UInt32(a + b as u32),
            (UInt32(a), Int32(b))     => UInt32(a + b as u32),
            (UInt32(a), Int64(b))     => UInt32(a + b as u32),
            (UInt32(a), UInt64(b))    => UInt32(a + b as u32),
            (UInt32(a), Float32(b))   => UInt32(a + b.round() as u32),
            (UInt32(a), Float64(b))   => UInt32(a + b.round() as u32),

            // Int64
            (Int64(a), Int64(b))      => Int64(a + b),
            (Int64(a), Int8(b))        => Int64(a + b as i64),
            (Int64(a), UInt8(b))       => Int64(a + b as i64),
            (Int64(a), Int16(b))      => Int64(a + b as i64),
            (Int64(a), UInt16(b))     => Int64(a + b as i64),
            (Int64(a), Int32(b))      => Int64(a + b as i64),
            (Int64(a), UInt32(b))     => Int64(a + b as i64),
            (Int64(a), UInt64(b))     => Int64(a + b as i64),
            (Int64(a), Float32(b))    => Int64(a + b.round() as i64),
            (Int64(a), Float64(b))    => Int64(a + b.round() as i64),

            // UInt64
            (UInt64(a), UInt64(b))    => UInt64(a + b),
            (UInt64(a), Int8(b))       => UInt64(a + b as u64),
            (UInt64(a), UInt8(b))      => UInt64(a + b as u64),
            (UInt64(a), Int16(b))     => UInt64(a + b as u64),
            (UInt64(a), UInt16(b))    => UInt64(a + b as u64),
            (UInt64(a), Int32(b))     => UInt64(a + b as u64),
            (UInt64(a), UInt32(b))    => UInt64(a + b as u64),
            (UInt64(a), Int64(b))     => UInt64(a + b as u64),
            (UInt64(a), Float32(b))   => UInt64(a + b.round() as u64),
            (UInt64(a), Float64(b))   => UInt64(a + b.round() as u64),

            // Float32s
            (Float32(a), Float32(b))  => Float32(a + b),
            (Float32(a), Int8(b))      => Float32(a + b as f32),
            (Float32(a), UInt8(b))     => Float32(a + b as f32),
            (Float32(a), Int16(b))    => Float32(a + b as f32),
            (Float32(a), UInt16(b))   => Float32(a + b as f32),
            (Float32(a), Int32(b))    => Float32(a + b as f32),
            (Float32(a), UInt32(b))   => Float32(a + b as f32),
            (Float32(a), Int64(b))    => Float32(a + b as f32),
            (Float32(a), UInt64(b))   => Float32(a + b as f32),
            (Float32(a), Float64(b))  => Float32(a + b as f32),

            // Float64s
            (Float64(a), Float64(b))  => Float64(a + b),
            (Float64(a), Int8(b))      => Float64(a + b as f64),
            (Float64(a), UInt8(b))     => Float64(a + b as f64),
            (Float64(a), Int16(b))    => Float64(a + b as f64),
            (Float64(a), UInt16(b))   => Float64(a + b as f64),
            (Float64(a), Int32(b))    => Float64(a + b as f64),
            (Float64(a), UInt32(b))   => Float64(a + b as f64),
            (Float64(a), Int64(b))    => Float64(a + b as f64),
            (Float64(a), UInt64(b))   => Float64(a + b as f64),
            (Float64(a), Float32(b))  => Float64(a + b as f64),
            _ => {
                eprintln!("Add not supported between given types: {:?} and {:?}", l, r);
                Value::Null
            }
        },

        BinaryOperator::Subtract => match (l.clone(), r.clone()) {

            // Int8
            (Int8(a), Int8(b))          => Int8(a - b),
            (Int8(a), UInt8(b))         => Int8(a - b as i8),
            (Int8(a), Int16(b))        => Int8(a - b as i8),
            (Int8(a), UInt16(b))       => Int8(a - b as i8),
            (Int8(a), Int32(b))        => Int8(a - b as i8),
            (Int8(a), UInt32(b))       => Int8(a - b as i8),
            (Int8(a), Int64(b))        => Int8(a - b as i8),
            (Int8(a), UInt64(b))       => Int8(a - b as i8),
            (Int8(a), Float32(b))      => Int8(a - b.round() as i8),
            (Int8(a), Float64(b))      => Int8(a - b.round() as i8),

            // Uint8
            (UInt8(a), UInt8(b))        => UInt8(a - b),
            (UInt8(a), Int8(b))         => UInt8(a - b as u8),
            (UInt8(a), Int16(b))       => UInt8(a - b as u8),
            (UInt8(a), UInt16(b))      => UInt8(a - b as u8),
            (UInt8(a), Int32(b))       => UInt8(a - b as u8),
            (UInt8(a), UInt32(b))      => UInt8(a - b as u8),
            (UInt8(a), Int64(b))       => UInt8(a - b as u8),
            (UInt8(a), UInt64(b))      => UInt8(a - b as u8),
            (UInt8(a), Float32(b))     => UInt8(a - b.round() as u8),
            (UInt8(a), Float64(b))     => UInt8(a - b.round() as u8),

            // Int16
            (Int16(a), Int16(b))      => Int16(a - b),
            (Int16(a), Int8(b))        => Int16(a - b as i16),
            (Int16(a), UInt8(b))       => Int16(a - b as i16),
            (Int16(a), UInt16(b))     => Int16(a - b as i16),
            (Int16(a), Int32(b))      => Int16(a - b as i16),
            (Int16(a), UInt32(b))     => Int16(a - b as i16),
            (Int16(a), Int64(b))      => Int16(a - b as i16),
            (Int16(a), UInt64(b))     => Int16(a - b as i16),
            (Int16(a), Float32(b))    => Int16(a - b.round() as i16),
            (Int16(a), Float64(b))    => Int16(a - b.round() as i16),

            // UInt16
            (UInt16(a), UInt16(b))    => UInt16(a - b),
            (UInt16(a), Int8(b))       => UInt16(a - b as u16),
            (UInt16(a), UInt8(b))      => UInt16(a - b as u16),
            (UInt16(a), Int16(b))     => UInt16(a - b as u16),
            (UInt16(a), Int32(b))     => UInt16(a - b as u16),
            (UInt16(a), UInt32(b))    => UInt16(a - b as u16),
            (UInt16(a), Int64(b))     => UInt16(a - b as u16),
            (UInt16(a), UInt64(b))    => UInt16(a - b as u16),
            (UInt16(a), Float32(b))   => UInt16(a - b.round() as u16),
            (UInt16(a), Float64(b))   => UInt16(a - b.round() as u16),

            // Int32
            (Int32(a), Int32(b))      => Int32(a - b),
            (Int32(a), Int8(b))        => Int32(a - b as i32),
            (Int32(a), UInt8(b))       => Int32(a - b as i32),
            (Int32(a), Int16(b))      => Int32(a - b as i32),
            (Int32(a), UInt16(b))     => Int32(a - b as i32),
            (Int32(a), UInt32(b))     => Int32(a - b as i32),
            (Int32(a), Int64(b))      => Int32(a - b as i32),
            (Int32(a), UInt64(b))     => Int32(a - b as i32),
            (Int32(a), Float32(b))    => Int32(a - b.round() as i32),
            (Int32(a), Float64(b))    => Int32(a - b.round() as i32),

            // UInt32
            (UInt32(a), UInt32(b))    => UInt32(a - b),
            (UInt32(a), Int8(b))       => UInt32(a - b as u32),
            (UInt32(a), UInt8(b))      => UInt32(a - b as u32),
            (UInt32(a), Int16(b))     => UInt32(a - b as u32),
            (UInt32(a), UInt16(b))    => UInt32(a - b as u32),
            (UInt32(a), Int32(b))     => UInt32(a - b as u32),
            (UInt32(a), Int64(b))     => UInt32(a - b as u32),
            (UInt32(a), UInt64(b))    => UInt32(a - b as u32),
            (UInt32(a), Float32(b))   => UInt32(a - b.round() as u32),
            (UInt32(a), Float64(b))   => UInt32(a - b.round() as u32),

            // Int64
            (Int64(a), Int64(b))      => Int64(a - b),
            (Int64(a), Int8(b))        => Int64(a - b as i64),
            (Int64(a), UInt8(b))       => Int64(a - b as i64),
            (Int64(a), Int16(b))      => Int64(a - b as i64),
            (Int64(a), UInt16(b))     => Int64(a - b as i64),
            (Int64(a), Int32(b))      => Int64(a - b as i64),
            (Int64(a), UInt32(b))     => Int64(a - b as i64),
            (Int64(a), UInt64(b))     => Int64(a - b as i64),
            (Int64(a), Float32(b))    => Int64(a - b.round() as i64),
            (Int64(a), Float64(b))    => Int64(a - b.round() as i64),

            // UInt64
            (UInt64(a), UInt64(b))    => UInt64(a - b),
            (UInt64(a), Int8(b))       => UInt64(a - b as u64),
            (UInt64(a), UInt8(b))      => UInt64(a - b as u64),
            (UInt64(a), Int16(b))     => UInt64(a - b as u64),
            (UInt64(a), UInt16(b))    => UInt64(a - b as u64),
            (UInt64(a), Int32(b))     => UInt64(a - b as u64),
            (UInt64(a), UInt32(b))    => UInt64(a - b as u64),
            (UInt64(a), Int64(b))     => UInt64(a - b as u64),
            (UInt64(a), Float32(b))   => UInt64(a - b.round() as u64),
            (UInt64(a), Float64(b))   => UInt64(a - b.round() as u64),

            // Float32s
            (Float32(a), Float32(b))  => Float32(a - b),
            (Float32(a), Int8(b))      => Float32(a - b as f32),
            (Float32(a), UInt8(b))     => Float32(a - b as f32),
            (Float32(a), Int16(b))    => Float32(a - b as f32),
            (Float32(a), UInt16(b))   => Float32(a - b as f32),
            (Float32(a), Int32(b))    => Float32(a - b as f32),
            (Float32(a), UInt32(b))   => Float32(a - b as f32),
            (Float32(a), Int64(b))    => Float32(a - b as f32),
            (Float32(a), UInt64(b))   => Float32(a - b as f32),
            (Float32(a), Float64(b))  => Float32(a - b as f32),

            // Float64s
            (Float64(a), Float64(b))  => Float64(a - b),
            (Float64(a), Int8(b))      => Float64(a - b as f64),
            (Float64(a), UInt8(b))     => Float64(a - b as f64),
            (Float64(a), Int16(b))    => Float64(a - b as f64),
            (Float64(a), UInt16(b))   => Float64(a - b as f64),
            (Float64(a), Int32(b))    => Float64(a - b as f64),
            (Float64(a), UInt32(b))   => Float64(a - b as f64),
            (Float64(a), Int64(b))    => Float64(a - b as f64),
            (Float64(a), UInt64(b))   => Float64(a - b as f64),
            (Float64(a), Float32(b))  => Float64(a - b as f64),


            _ => {
                eprintln!("Subtract not supported between given types: {:?} - {:?}.", l, r);
                Value::Null
            }
        },

        BinaryOperator::Multiply => match (l.clone(), r.clone()) {
            // Int8
            (Int8(a), Int8(b))          => Int8(a * b),
            (Int8(a), UInt8(b))         => Int8(a * b as i8),
            (Int8(a), Int16(b))        => Int8(a * b as i8),
            (Int8(a), UInt16(b))       => Int8(a * b as i8),
            (Int8(a), Int32(b))        => Int8(a * b as i8),
            (Int8(a), UInt32(b))       => Int8(a * b as i8),
            (Int8(a), Int64(b))        => Int8(a * b as i8),
            (Int8(a), UInt64(b))       => Int8(a * b as i8),
            (Int8(a), Float32(b))      => Int8(a * b.round() as i8),
            (Int8(a), Float64(b))      => Int8(a * b.round() as i8),

            // Uint8
            (UInt8(a), UInt8(b))        => UInt8(a * b),
            (UInt8(a), Int8(b))         => UInt8(a * b as u8),
            (UInt8(a), Int16(b))       => UInt8(a * b as u8),
            (UInt8(a), UInt16(b))      => UInt8(a * b as u8),
            (UInt8(a), Int32(b))       => UInt8(a * b as u8),
            (UInt8(a), UInt32(b))      => UInt8(a * b as u8),
            (UInt8(a), Int64(b))       => UInt8(a * b as u8),
            (UInt8(a), UInt64(b))      => UInt8(a * b as u8),
            (UInt8(a), Float32(b))     => UInt8(a * b.round() as u8),
            (UInt8(a), Float64(b))     => UInt8(a * b.round() as u8),

            // Int16
            (Int16(a), Int16(b))      => Int16(a * b),
            (Int16(a), Int8(b))        => Int16(a * b as i16),
            (Int16(a), UInt8(b))       => Int16(a * b as i16),
            (Int16(a), UInt16(b))     => Int16(a * b as i16),
            (Int16(a), Int32(b))      => Int16(a * b as i16),
            (Int16(a), UInt32(b))     => Int16(a * b as i16),
            (Int16(a), Int64(b))      => Int16(a * b as i16),
            (Int16(a), UInt64(b))     => Int16(a * b as i16),
            (Int16(a), Float32(b))    => Int16(a * b.round() as i16),
            (Int16(a), Float64(b))    => Int16(a * b.round() as i16),

            // UInt16
            (UInt16(a), UInt16(b))    => UInt16(a * b),
            (UInt16(a), Int8(b))       => UInt16(a * b as u16),
            (UInt16(a), UInt8(b))      => UInt16(a * b as u16),
            (UInt16(a), Int16(b))     => UInt16(a * b as u16),
            (UInt16(a), Int32(b))     => UInt16(a * b as u16),
            (UInt16(a), UInt32(b))    => UInt16(a * b as u16),
            (UInt16(a), Int64(b))     => UInt16(a * b as u16),
            (UInt16(a), UInt64(b))    => UInt16(a * b as u16),
            (UInt16(a), Float32(b))   => UInt16(a * b.round() as u16),
            (UInt16(a), Float64(b))   => UInt16(a * b.round() as u16),

            // Int32
            (Int32(a), Int32(b))      => Int32(a * b),
            (Int32(a), Int8(b))        => Int32(a * b as i32),
            (Int32(a), UInt8(b))       => Int32(a * b as i32),
            (Int32(a), Int16(b))      => Int32(a * b as i32),
            (Int32(a), UInt16(b))     => Int32(a * b as i32),
            (Int32(a), UInt32(b))     => Int32(a * b as i32),
            (Int32(a), Int64(b))      => Int32(a * b as i32),
            (Int32(a), UInt64(b))     => Int32(a * b as i32),
            (Int32(a), Float32(b))    => Int32(a * b.round() as i32),
            (Int32(a), Float64(b))    => Int32(a * b.round() as i32),

            // UInt32
            (UInt32(a), UInt32(b))    => UInt32(a * b),
            (UInt32(a), Int8(b))       => UInt32(a * b as u32),
            (UInt32(a), UInt8(b))      => UInt32(a * b as u32),
            (UInt32(a), Int16(b))     => UInt32(a * b as u32),
            (UInt32(a), UInt16(b))    => UInt32(a * b as u32),
            (UInt32(a), Int32(b))     => UInt32(a * b as u32),
            (UInt32(a), Int64(b))     => UInt32(a * b as u32),
            (UInt32(a), UInt64(b))    => UInt32(a * b as u32),
            (UInt32(a), Float32(b))   => UInt32(a * b.round() as u32),
            (UInt32(a), Float64(b))   => UInt32(a * b.round() as u32),

            // Int64
            (Int64(a), Int64(b))      => Int64(a * b),
            (Int64(a), Int8(b))        => Int64(a * b as i64),
            (Int64(a), UInt8(b))       => Int64(a * b as i64),
            (Int64(a), Int16(b))      => Int64(a * b as i64),
            (Int64(a), UInt16(b))     => Int64(a * b as i64),
            (Int64(a), Int32(b))      => Int64(a * b as i64),
            (Int64(a), UInt32(b))     => Int64(a * b as i64),
            (Int64(a), UInt64(b))     => Int64(a * b as i64),
            (Int64(a), Float32(b))    => Int64(a * b.round() as i64),
            (Int64(a), Float64(b))    => Int64(a * b.round() as i64),

            // UInt64
            (UInt64(a), UInt64(b))    => UInt64(a * b),
            (UInt64(a), Int8(b))       => UInt64(a * b as u64),
            (UInt64(a), UInt8(b))      => UInt64(a * b as u64),
            (UInt64(a), Int16(b))     => UInt64(a * b as u64),
            (UInt64(a), UInt16(b))    => UInt64(a * b as u64),
            (UInt64(a), Int32(b))     => UInt64(a * b as u64),
            (UInt64(a), UInt32(b))    => UInt64(a * b as u64),
            (UInt64(a), Int64(b))     => UInt64(a * b as u64),
            (UInt64(a), Float32(b))   => UInt64(a * b.round() as u64),
            (UInt64(a), Float64(b))   => UInt64(a * b.round() as u64),

            // Float32s
            (Float32(a), Float32(b))  => Float32(a * b),
            (Float32(a), Int8(b))      => Float32(a * b as f32),
            (Float32(a), UInt8(b))     => Float32(a * b as f32),
            (Float32(a), Int16(b))    => Float32(a * b as f32),
            (Float32(a), UInt16(b))   => Float32(a * b as f32),
            (Float32(a), Int32(b))    => Float32(a * b as f32),
            (Float32(a), UInt32(b))   => Float32(a * b as f32),
            (Float32(a), Int64(b))    => Float32(a * b as f32),
            (Float32(a), UInt64(b))   => Float32(a * b as f32),
            (Float32(a), Float64(b))  => Float32(a * b as f32),

            // Float64s
            (Float64(a), Float64(b))  => Float64(a * b),
            (Float64(a), Int8(b))      => Float64(a * b as f64),
            (Float64(a), UInt8(b))     => Float64(a * b as f64),
            (Float64(a), Int16(b))    => Float64(a * b as f64),
            (Float64(a), UInt16(b))   => Float64(a * b as f64),
            (Float64(a), Int32(b))    => Float64(a * b as f64),
            (Float64(a), UInt32(b))   => Float64(a * b as f64),
            (Float64(a), Int64(b))    => Float64(a * b as f64),
            (Float64(a), UInt64(b))   => Float64(a * b as f64),
            (Float64(a), Float32(b))  => Float64(a * b as f64),

            _ => {
            eprintln!("Subtract not supported between given types: {:?} - {:?}.", l, r);
            Value::Null
            }
        },

        BinaryOperator::Divide => match (l.clone(), r.clone()) {
            // Int8
            (Int8(a), Int8(b))          => Int8(a / b),
            (Int8(a), UInt8(b))         => Int8(a / b as i8),
            (Int8(a), Int16(b))        => Int8(a / b as i8),
            (Int8(a), UInt16(b))       => Int8(a / b as i8),
            (Int8(a), Int32(b))        => Int8(a / b as i8),
            (Int8(a), UInt32(b))       => Int8(a / b as i8),
            (Int8(a), Int64(b))        => Int8(a / b as i8),
            (Int8(a), UInt64(b))       => Int8(a / b as i8),
            (Int8(a), Float32(b))      => Int8(a / b.round() as i8),
            (Int8(a), Float64(b))      => Int8(a / b.round() as i8),

            // Uint8
            (UInt8(a), UInt8(b))        => UInt8(a / b),
            (UInt8(a), Int8(b))         => UInt8(a / b as u8),
            (UInt8(a), Int16(b))       => UInt8(a / b as u8),
            (UInt8(a), UInt16(b))      => UInt8(a / b as u8),
            (UInt8(a), Int32(b))       => UInt8(a / b as u8),
            (UInt8(a), UInt32(b))      => UInt8(a / b as u8),
            (UInt8(a), Int64(b))       => UInt8(a / b as u8),
            (UInt8(a), UInt64(b))      => UInt8(a / b as u8),
            (UInt8(a), Float32(b))     => UInt8(a / b.round() as u8),
            (UInt8(a), Float64(b))     => UInt8(a / b.round() as u8),

            // Int16
            (Int16(a), Int16(b))      => Int16(a / b),
            (Int16(a), Int8(b))        => Int16(a / b as i16),
            (Int16(a), UInt8(b))       => Int16(a / b as i16),
            (Int16(a), UInt16(b))     => Int16(a / b as i16),
            (Int16(a), Int32(b))      => Int16(a / b as i16),
            (Int16(a), UInt32(b))     => Int16(a / b as i16),
            (Int16(a), Int64(b))      => Int16(a / b as i16),
            (Int16(a), UInt64(b))     => Int16(a / b as i16),
            (Int16(a), Float32(b))    => Int16(a / b.round() as i16),
            (Int16(a), Float64(b))    => Int16(a / b.round() as i16),

            // UInt16
            (UInt16(a), UInt16(b))    => UInt16(a / b),
            (UInt16(a), Int8(b))       => UInt16(a / b as u16),
            (UInt16(a), UInt8(b))      => UInt16(a / b as u16),
            (UInt16(a), Int16(b))     => UInt16(a / b as u16),
            (UInt16(a), Int32(b))     => UInt16(a / b as u16),
            (UInt16(a), UInt32(b))    => UInt16(a / b as u16),
            (UInt16(a), Int64(b))     => UInt16(a / b as u16),
            (UInt16(a), UInt64(b))    => UInt16(a / b as u16),
            (UInt16(a), Float32(b))   => UInt16(a / b.round() as u16),
            (UInt16(a), Float64(b))   => UInt16(a / b.round() as u16),

            // Int32
            (Int32(a), Int32(b))      => Int32(a / b),
            (Int32(a), Int8(b))        => Int32(a / b as i32),
            (Int32(a), UInt8(b))       => Int32(a / b as i32),
            (Int32(a), Int16(b))      => Int32(a / b as i32),
            (Int32(a), UInt16(b))     => Int32(a / b as i32),
            (Int32(a), UInt32(b))     => Int32(a / b as i32),
            (Int32(a), Int64(b))      => Int32(a / b as i32),
            (Int32(a), UInt64(b))     => Int32(a / b as i32),
            (Int32(a), Float32(b))    => Int32(a / b.round() as i32),
            (Int32(a), Float64(b))    => Int32(a / b.round() as i32),

            // UInt32
            (UInt32(a), UInt32(b))    => UInt32(a / b),
            (UInt32(a), Int8(b))       => UInt32(a / b as u32),
            (UInt32(a), UInt8(b))      => UInt32(a / b as u32),
            (UInt32(a), Int16(b))     => UInt32(a / b as u32),
            (UInt32(a), UInt16(b))    => UInt32(a / b as u32),
            (UInt32(a), Int32(b))     => UInt32(a / b as u32),
            (UInt32(a), Int64(b))     => UInt32(a / b as u32),
            (UInt32(a), UInt64(b))    => UInt32(a / b as u32),
            (UInt32(a), Float32(b))   => UInt32(a / b.round() as u32),
            (UInt32(a), Float64(b))   => UInt32(a / b.round() as u32),

            // Int64
            (Int64(a), Int64(b))      => Int64(a / b),
            (Int64(a), Int8(b))        => Int64(a / b as i64),
            (Int64(a), UInt8(b))       => Int64(a / b as i64),
            (Int64(a), Int16(b))      => Int64(a / b as i64),
            (Int64(a), UInt16(b))     => Int64(a / b as i64),
            (Int64(a), Int32(b))      => Int64(a / b as i64),
            (Int64(a), UInt32(b))     => Int64(a / b as i64),
            (Int64(a), UInt64(b))     => Int64(a / b as i64),
            (Int64(a), Float32(b))    => Int64(a / b.round() as i64),
            (Int64(a), Float64(b))    => Int64(a / b.round() as i64),

            // UInt64
            (UInt64(a), UInt64(b))    => UInt64(a / b),
            (UInt64(a), Int8(b))       => UInt64(a / b as u64),
            (UInt64(a), UInt8(b))      => UInt64(a / b as u64),
            (UInt64(a), Int16(b))     => UInt64(a / b as u64),
            (UInt64(a), UInt16(b))    => UInt64(a / b as u64),
            (UInt64(a), Int32(b))     => UInt64(a / b as u64),
            (UInt64(a), UInt32(b))    => UInt64(a / b as u64),
            (UInt64(a), Int64(b))     => UInt64(a / b as u64),
            (UInt64(a), Float32(b))   => UInt64(a / b.round() as u64),
            (UInt64(a), Float64(b))   => UInt64(a / b.round() as u64),

            // Float32s
            (Float32(a), Float32(b))  => Float32(a / b),
            (Float32(a), Int8(b))      => Float32(a / b as f32),
            (Float32(a), UInt8(b))     => Float32(a / b as f32),
            (Float32(a), Int16(b))    => Float32(a / b as f32),
            (Float32(a), UInt16(b))   => Float32(a / b as f32),
            (Float32(a), Int32(b))    => Float32(a / b as f32),
            (Float32(a), UInt32(b))   => Float32(a / b as f32),
            (Float32(a), Int64(b))    => Float32(a / b as f32),
            (Float32(a), UInt64(b))   => Float32(a / b as f32),
            (Float32(a), Float64(b))  => Float32(a / b as f32),

            // Float64s
            (Float64(a), Float64(b))  => Float64(a / b),
            (Float64(a), Int8(b))      => Float64(a / b as f64),
            (Float64(a), UInt8(b))     => Float64(a / b as f64),
            (Float64(a), Int16(b))    => Float64(a / b as f64),
            (Float64(a), UInt16(b))   => Float64(a / b as f64),
            (Float64(a), Int32(b))    => Float64(a / b as f64),
            (Float64(a), UInt32(b))   => Float64(a / b as f64),
            (Float64(a), Int64(b))    => Float64(a / b as f64),
            (Float64(a), UInt64(b))   => Float64(a / b as f64),
            (Float64(a), Float32(b))  => Float64(a / b as f64),

            _ => {
            eprintln!("Subtract not supported between given types: {:?} - {:?}.", l, r);
            Value::Null
            }
        }
        // Handle other ops similarly...
        _ => {
            eprintln!("Unsupported binary operator: {:?}", op);
            Value::Null
        }
    }
}
