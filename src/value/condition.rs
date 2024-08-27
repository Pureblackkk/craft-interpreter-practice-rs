use super::LValue;

pub trait IsTruthy {
    fn is_truthy(&self) -> bool;
}

impl IsTruthy for LValue {
    fn is_truthy(&self) -> bool {
        match self {
            LValue::Bool(bool) => *bool,
            LValue::Nil => false,
            LValue::Number(num) => *num != 0.0,
            LValue::String(str) => (*str).is_empty(),
            LValue::Function(_) => true,
            LValue::Class(_) => true,
            LValue::ClassInstance(_) => true,
        }
    }
}