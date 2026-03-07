use serde::{Serialize, Deserialize};
use thiserror::Error;
use std::collections::HashMap;
use once_cell::sync::Lazy;

use crate::errors::CustomError;

static SEPARATOR: Lazy<String> = Lazy::new(|| String::from("\r\n"));

#[derive(Clone, Debug)]
pub struct Header(HashMap<String, String>);

impl Header {

    fn new() -> Self {
        Header(HashMap::new())
    }

    fn parse_line<'a>(input_line: &'a str) -> Result<(&'a str, &'a str), CustomError> {

        let Some((key, value)) = input_line.split_once(":") else {
            return Err(CustomError::MalformedHeader(input_line.to_string()));
        };
        
        return Ok((key.trim(), value.trim()));
    }

    fn insert(&mut self, key: String, value: String) -> bool {
        self.0.insert(key, value);

        true
    }

    fn get(&self, key: &str) -> Option<&String> {
        self.0.get(key)
    }
}

pub async fn parse(input: Option<String>) -> Result<Header, CustomError> {
    
    let Some(input) = input else {
        return Err(CustomError::CustomErrorMessage("Empty Header List passed in".to_string()));
    };
    let mut header = Header::new();
    for item in input.split(&*SEPARATOR) {
        if item.is_empty() {
            break;
        }

        println!("{}", item);
        let (key, value) = Header::parse_line(&item)?;
        
        header.insert(key.to_string(), value.to_string());

    }

    Ok(header)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_parse_header() {
        let headers = parse(Some("Host: http://127.0.0.1:42062\r\nContent-Type:    text/html".to_string())).await.unwrap();
        println!("{:?}", headers);
        // assert_eq!(headers.get("Host"), Some(&"example.com".to_string()));
    }
}