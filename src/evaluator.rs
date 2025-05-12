use crate::parser::{Expression, BinaryOperator};
use crate::runtime::{Runtime, Value};
use crate::jadeErrors::{valueError, variableNotFoundError};

#[allow(dead_code, unreachable_patterns)]
pub fn evaluate(expr: &Expression, runtime: &Runtime, expected_type: Option<&str>) -> Value {
    let val = match expr {
        Expression::String(val) => Value::String(val.clone()),
        Expression::Number(val) => {
            val.to_string().parse::<f64>().map(Value::Float64).unwrap_or(Value::Null)
        }
        Expression::Boolean(val) => Value::Bool(*val),

        Expression::Variable(name) => {
            runtime.get_variable(name).cloned().unwrap_or_else(|| {
                variableNotFoundError(name);
                Value::Null
            })
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

    if let Some(t) = expected_type {
        coerce_to_type(val, t)
    } else {
        val
    }
}

fn coerce_to_type(val: Value, type_str: &str) -> Value {
    use Value::*;

    match (type_str, val) {
        ("$", Int8(v)) => Int8(v),
        ("$", Bool(b)) => Bool(b),
        ("$", UInt8(v)) => UInt8(v),
        ("$", Int16(v)) => Int16(v),
        ("$", UInt16(v)) => UInt16(v),
        ("$", Int32(v)) => Int32(v),
        ("$", UInt32(v)) => UInt32(v),
        ("$", Int64(v)) => Int64(v),
        ("$", UInt64(v)) => UInt64(v),
        ("$", Float32(v)) => Float32(v),
        ("$", Float64(v)) => Float64(v),
        ("$", String(s)) => String(s),
        ("int8", Int8(v)) => Int8(v),
        ("int8", UInt8(v)) => Int8(v as i8),
        ("int8", Float64(v)) => Int8(v as i8),
        ("uint8", Float64(v)) => UInt8(v as u8),
        ("uint8", UInt8(v)) => UInt8(v),
        ("float64", Int8(v)) => Float64(v as f64),
        ("float64", UInt8(v)) => Float64(v as f64),
        ("float64", Float64(v)) => Float64(v),
        ("string", String(s)) => String(s),
        ("bool", Bool(b)) => Bool(b),
        (_, other) => {
            valueError(type_str, other);
            Null
        }
    }
}

fn evaluate_binary_op(op: &BinaryOperator, l: Value, r: Value) -> Value {
    use Value::*;
    match op {
        BinaryOperator::Add => match (l, r) {
            (Int8(a), Int8(b)) => Int8(a + b),
            (UInt8(a), UInt8(b)) => UInt8(a + b),
            (Float64(a), Float64(b)) => Float64(a + b),
            (Int8(a), UInt8(b)) => Int8(a + b as i8),
            (UInt8(a), Int8(b)) => UInt8(a + b as u8),
            _ => {
                eprintln!("Add not supported between given types.");
                Value::Null
            }
        },
        // Handle other ops similarly...
        _ => {
            eprintln!("Unsupported binary operator: {:?}", op);
            Value::Null
        }
    }
}
