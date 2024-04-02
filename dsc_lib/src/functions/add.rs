// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use crate::DscError;
use crate::configure::context::Context;
use crate::functions::{AcceptedArgKind, Function};
use num_traits::cast::NumCast;
use serde_json::{Number, Value};
use tracing::debug;

#[derive(Debug, Default)]
pub struct Add {}

impl Function for Add {
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
        debug!("add function");
        if let (Some(arg1), Some(arg2)) = (args[0].as_f64(), args[1].as_f64()) {
            let i64_max: f64 = NumCast::from(i64::MAX).ok_or(DscError::Parser("Failed to convert from int to float".to_string()))?;
            if arg1 > i64_max || arg2 > i64_max {
                return Err(DscError::Parser("Addition input overflow".to_string()));
            }
            let val1: i64 = NumCast::from(arg1).ok_or(DscError::Parser("Invalid input".to_string()))?;
            let val2 = NumCast::from(arg2).ok_or(DscError::Parser("Invalid input".to_string()))?;
            if let Some(result) = val1.checked_add(val2) {
                Ok(Value::Number(Number::from(result)))
            } else {
                Err(DscError::Parser("Addition result overflow".to_string()))
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
        let result = parser.parse_and_execute("[add(2, 3)]", &Context::new()).unwrap();
        assert_eq!(result, 5);
    }

    #[test]
    fn nested() {
        let mut parser = Statement::new().unwrap();
        let result = parser.parse_and_execute("[add(2, add(3, 4))]", &Context::new()).unwrap();
        assert_eq!(result, 9);
    }

    #[test]
    fn invalid_one_parameter() {
        let mut parser = Statement::new().unwrap();
        let result = parser.parse_and_execute("[add(5)]", &Context::new());
        assert!(result.is_err());
    }

    #[test]
    fn overflow_result() {
        let mut parser = Statement::new().unwrap();
        // max value for i64 is 2^63 -1 (or 9,223,372,036,854,775,807)
        let result = parser.parse_and_execute("[add(9223372036854775807, 2)]", &Context::new());
        assert!(result.is_err());
    }

    #[test]
    fn overflow_input() {
        let mut parser = Statement::new().unwrap();
        let result = parser.parse_and_execute("[add(9223372036854785808, 0)]", &Context::new());
        println!("output {:?}", result);
        assert!(result.is_err());
    }
}
