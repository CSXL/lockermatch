use crate::http::Error;
use crate::redis::RedisPool;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct StudentId(String);

impl StudentId {
    pub fn new(id: String) -> Result<Self, Error> {
        if id.len() == 6 && id.chars().all(|c| c.is_digit(10)) {
            Ok(StudentId(id))
        } else {
            Err(Error::UnprocessableEntity {
                errors: HashMap::from([("id".into(), vec!["must be 6 digits".into()])]),
            })
        }
    }

    pub fn to_string(&self) -> String {
        self.0.clone()
    }

    pub fn from_string(id: String) -> Result<Self, Error> {
        return Self::new(id);
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::init_logging;
    use log::debug;

    fn setup() {
        init_logging().unwrap();
    }

    #[test]
    fn test_student_id_new() {
        setup();
        let id = StudentId::new("123456".to_string());
        assert_eq!(id.as_ref().unwrap().to_string(), "123456");
        debug!("id: {:?}", id);
    }

    #[test]
    fn test_student_id_new_invalid() {
        setup();
        let id = StudentId::new("12345".to_string());
        assert!(id.as_ref().is_err());
        debug!("error: {:?}", id.as_ref().err().unwrap());
    }
}
