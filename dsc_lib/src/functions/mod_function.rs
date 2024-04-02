// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use crate::DscError;
use crate::configure::context::Context;
use crate::functions::{AcceptedArgKind, Function};
use num_traits::cast::NumCast;
use serde_json::{Number, Value};
use tracing::debug;

#[derive(Debug, Default)]
pub struct Mod {}

impl Function for Mod {
    fn min_args(&self) -> usize {
        2
    }

    fn max_args(&self) -> usize {
        2
    }

    fn accepted_arg_types(&self) -> Vec<AcceptedArgKind> {
        vec![AcceptedArgKind::Number]
    }

    fn invoke(&self, args: &[Value], _context: &Context) -> Result<Value, DscError> {
        debug!("mod function");
        if let (Some(arg1), Some(arg2)) = (args[0].as_f64(), args[1].as_f64()) {
            let i64_max: f64 = NumCast::from(i64::MAX).ok_or(DscError::Parser("Failed to convert from int to float".to_string()))?;
            if arg1 > i64_max || arg2 > i64_max {
                return Err(DscError::Parser("Mod input overflow".to_string()));
            }
            let val1: i64 = NumCast::from(arg1).ok_or(DscError::Parser("Invalid input".to_string()))?;
            let val2: i64 = NumCast::from(arg2).ok_or(DscError::Parser("Invalid input".to_string()))?;
            if let Some(value) = val1.checked_rem(val2) {
                Ok(Value::Number(Number::from(value)))
            } else {
                Err(DscError::Parser("Division for mod results in overflow".to_string()))
            }
        } else {
            Err(DscError::Parser("Invalid argument(s)".to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::configure::context::Context;
    use crate::parser::Statement;

    #[test]
    fn numbers() {
        let mut parser = Statement::new().unwrap();
        let result = parser.parse_and_execute("[mod(7, 3)]", &Context::new()).unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    fn nested() {
        let mut parser = Statement::new().unwrap();
        let result = parser.parse_and_execute("[mod(18, mod(8, 3))]", &Context::new()).unwrap();
        assert_eq!(result, 0);
    }

    #[test]
    fn invalid_one_parameter() {
        let mut parser = Statement::new().unwrap();
        let result = parser.parse_and_execute("[mod(5)]", &Context::new());
        assert!(result.is_err());
    }

    #[test]
    fn invalid_div_by_zero() {
        let mut parser = Statement::new().unwrap();
        let result = parser.parse_and_execute("[mod(5, 0)]", &Context::new());
        assert!(result.is_err());
    }

    #[test]
    fn overflow_input() {
        let mut parser = Statement::new().unwrap();
        // max value for i64 is 2^63 -1 (or 9,223,372,036,854,775,807)
        let result = parser.parse_and_execute("[mod(9223372036854775808, 2)]", &Context::new());
        assert!(result.is_err());
    }
}
