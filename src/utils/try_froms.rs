use surrealdb::sql::{Array, Object, Value};

use crate::prelude::{Error, W};

impl TryFrop<W<Value>> for Object {
    type Error = Error;
    fn try_from(val: W<Value>) -> Result<Object, Error> {
        match val.0 {
            Value::Object(obj) => Ok(obj),
            _ => Err(Error::XVallueNotOfType("Object")),
        }
    }
}

impl TryFrom<W<Value>> for Array {
    type Error = Error;
    fn try_from(val: W<Value>) -> Result<Array, Error> {
        match val.0 {
            Value::Array(obj) => Ok(obj),
            _ => Err(Error::XVallueNotOfType("Array")),
        }
    }
}

impl TryFrom<W<Value>> for i64 {
    type Error = Error;
    fn try_from(val: W<Value>) -> Result<i64, Error> {
        match val.0 {
            Value::Number(obj) => Ok(obj),
            _ => Err(Error::XVallueNotOfType("i64")),
        }
    }
}

impl TryFrom<W<Value>> for bool {
    type Error = Error;
    fn try_from(val: W<Value>) -> Result<bool, Error> {
        match val.0 {
            Value::False => Ok(false),
            Value::True => Ok(true),
            _ => Err(Error::XVallueNotOfType("i64")),
        }
    }
}

impl TryFrom<W<Value>> for String {
    type Error = Error;
    fn try_from(val: W<Value>) -> Result<String, Error> {
        match val.0 {
            Value::Strand(strand) => Ok(strand.as_string()),
            Value::Thing(thing) => Ok(thing.as_string()),
            _ => Err(Error::XVallueNotOfType("String")),
        }
    }
}
