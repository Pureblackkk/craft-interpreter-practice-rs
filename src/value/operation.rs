use super::LValue;
use std::ops;

// Unary Operation negative: -
impl ops::Neg for LValue {
    type Output = Result<LValue, String>;

    fn neg(self) -> Self::Output {
        match self {
            LValue::Number(n) => Ok(LValue::Number(-n)),
            _ => Err(format!("Invalid negative operation for type {:?}", self)),
        }
    }
}

// Unary Operation not: ! 
impl ops::Not for LValue {
    type Output = Result<LValue, String>;

    fn not(self) -> Self::Output {
        match self {
            LValue::Bool(bool) => Ok(LValue::Bool(!bool)),
            LValue::Nil => Ok(LValue::Bool(true)),
            _ => Ok(LValue::Bool(false)),
        }
    }
}


// Binary Opearation add: +
impl ops::Add<LValue> for LValue {
    type Output = Result<LValue, String>;

    fn add(self, rhs: LValue) -> Self::Output {
        match self {
            LValue::Number(l) => {
                match rhs {
                    LValue::Number(r) => Ok(LValue::Number(l + r)),
                    LValue::String(r) => Ok(LValue::String(l.to_string() + &r)),
                    LValue::Nil => Ok(LValue::Number(l)),
                    LValue::Bool(r) => Ok(LValue::Number(l + f64::from(r))),
                    LValue::Function(_) => Err(String::from("Operation add not supported for function")),
                    LValue::Class(_) => Err(String::from("Operation add not supported for class")),
                    LValue::ClassInstance(_) => Err(String::from("Operation add not supported for class instance")),
                }
            },
            LValue::String(l) => {
                match rhs {
                    LValue::Number(r) => Ok(LValue::String(l + r.to_string().as_str())),
                    LValue::String(r) =>  Ok(LValue::String(l + &r)),
                    LValue::Nil => Ok(LValue::String(l + "nil")),
                    LValue::Bool(r) => Ok(LValue::String(l + r.to_string().as_str())),
                    LValue::Function(_) => Err(String::from("Operation add not supported for function")),
                    LValue::Class(_) => Err(String::from("Operation add not supported for class")),
                    LValue::ClassInstance(_) => Err(String::from("Operation add not supported for class instance")),
                }
            },
            LValue::Bool(l) => {
                match rhs {
                    LValue::Number(r) => Ok(LValue::Number(f64::from(l) + r)),
                    LValue::String(r) => Ok(LValue::String(l.to_string() + &r)),
                    LValue::Nil => Ok(LValue::Number(f64::from(l))),
                    LValue::Bool(r) => Ok(LValue::Number(f64::from(l) + f64::from(r))),
                    LValue::Function(_) => Err(String::from("Operation add not supported for function")),
                    LValue::Class(_) => Err(String::from("Operation add not supported for class")),
                    LValue::ClassInstance(_) => Err(String::from("Operation add not supported for class instance")),
                }
            },
            LValue::Nil => {
                match rhs {
                    LValue::Number(r) => Ok(LValue::Number(r)),
                    LValue::String(r) => Ok(LValue::String(String::from("nil") + r.as_str())),
                    LValue::Nil => Ok(LValue::Number(0.0)),
                    LValue::Bool(r) => Ok(LValue::Number(f64::from(r))),
                    LValue::Function(_) => Err(String::from("Operation add not supported for function")),
                    LValue::Class(_) => Err(String::from("Operation add not supported for class")),
                    LValue::ClassInstance(_) => Err(String::from("Operation add not supported for class instance")),
                }
            },
            LValue::Function(_) => Err(String::from("Operation add not supported for function")),
            LValue::Class(_) => Err(String::from("Operation add not supported for class")),
            LValue::ClassInstance(_) => Err(String::from("Operation add not supported for class instance")),
        }
    }
}

// Binary Opearation subtract: -
impl ops::Sub<LValue> for LValue {
    type Output = Result<LValue, String>;

    fn sub(self, rhs: LValue) -> Self::Output {
        match self {
            LValue::Number(l) => {
                match rhs {
                    LValue::Number(r) => Ok(LValue::Number(l - r)),
                    LValue::String(_) => Err(String::from("Invalid operation subtract between number and string")),
                    LValue::Nil => Ok(LValue::Number(l)),
                    LValue::Bool(r) => Ok(LValue::Number(l - f64::from(r))),
                    LValue::Function(_) => Err(String::from("Operation sub not supported for function")),
                    LValue::Class(_) => Err(String::from("Operation sub not supported for class")),
                    LValue::ClassInstance(_) => Err(String::from("Operation sub not supported for class instance")),
                }
            },
            LValue::String(_) => {
                panic!("String is not support for subtraction")
            },
            LValue::Bool(l) => {
                match rhs {
                    LValue::Number(r) => Ok(LValue::Number(f64::from(l) - r)),
                    LValue::String(_) => Err(String::from("Invalid operation subtract between bool and string")),
                    LValue::Nil => Ok(LValue::Number(f64::from(l))),
                    LValue::Bool(r) => Ok(LValue::Number(f64::from(l) - f64::from(r))),
                    LValue::Function(_) => Err(String::from("Operation sub not supported for function")),
                    LValue::Class(_) => Err(String::from("Operation sub not supported for class")),
                    LValue::ClassInstance(_) => Err(String::from("Operation sub not supported for class instance")),
                }
            },
            LValue::Nil => {
                match rhs {
                    LValue::Number(r) => Ok(LValue::Number(-r)),
                    LValue::String(_) => Err(String::from("Invalid operation subtract between Nil and string")),
                    LValue::Nil => Ok(LValue::Number(0.0)),
                    LValue::Bool(r) => Ok(LValue::Number(0.0 - f64::from(r))),
                    LValue::Function(_) => Err(String::from("Operation sub not supported for function")),
                    LValue::Class(_) => Err(String::from("Operation sub not supported for class")),
                    LValue::ClassInstance(_) => Err(String::from("Operation sub not supported for class instance")),
                }
            },
            LValue::Function(_) => Err(String::from("Operation sub not supported for function")),
            LValue::Class(_) => Err(String::from("Operation sub not supported for class")),
            LValue::ClassInstance(_) => Err(String::from("Operation sub not supported for class instance")),
        }
    }
}

// Binary Operation multiple: *
impl ops::Mul<LValue> for LValue {
    type Output = Result<LValue, String>;

    fn mul(self, rhs: LValue) -> Self::Output {
        match self {
            LValue::Number(l) => {
                match rhs {
                    LValue::Number(r) => Ok(LValue::Number(l * r)),
                    _ => Err(String::from("Operation multiple only supports for Number")),
                }
            },
            _ => Err(String::from("Operation multiple only supports for Number")),
        }
    }
}

// Binary Operation divide: /
impl ops::Div<LValue> for LValue {
    type Output = Result<LValue, String>;

    fn div(self, rhs: LValue) -> Self::Output {
        match self {
            LValue::Number(l) => {
                match rhs {
                    LValue::Number(r) => Ok(LValue::Number(l / r)),
                    _ => Err(String::from("Operation divide only supports for Number")),
                }
            },
            _ => Err(String::from("Operation divide only supports for Number")),
        }
    }
}