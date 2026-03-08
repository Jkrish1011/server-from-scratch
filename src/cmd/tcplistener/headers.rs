use serde::{Serialize, Deserialize};
use thiserror::Error;
use std::collections::HashMap;
use once_cell::sync::Lazy;
use regex::Regex;
use tracing::{
    info, error
};

use crate::errors::CustomError;

static SEPARATOR: Lazy<String> = Lazy::new(|| String::from("\r\n"));

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Header(HashMap<String, String>);

impl Header {

    pub fn new() -> Self {
        Header(HashMap::new())
    }

    pub fn parse_line<'a>(input_line: &'a str) -> Result<(&'a str, &'a str), CustomError> {

        let Some((key, value)) = input_line.split_once(":") else {
            return Err(CustomError::MalformedHeader(input_line.to_string()));
        };
        
        return Ok((key.trim(), value.trim()));
    }

    pub fn insert(&mut self, key: String, value: String) -> bool {

        if let Some(existing_value) = self.get(&key) {
            let new_value = format!("{existing_value},{value}");
            self.0.insert(key, new_value);    
        } else {
            self.0.insert(key, value);
        };
        true
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.0.get(key)
    }

    pub fn is_valid_token(&self) -> Result<bool, CustomError> {
        let regex = Regex::new(r"^[a-zA-Z0-9!#\$%^&\*\+\-\.\|`~/\\:,]+$").unwrap();
        for (idx, item) in (&self.0).into_iter() {
            if !regex.is_match(&item) {
                return Err(CustomError::CustomErrorMessage(format!("Invalid Header : {}", item)));
            }
        }

        return Ok(true);
    }
}

pub async fn parse_header(input: Option<String>) -> Result<(Header, i32), CustomError> {
    info!("Parsing the headers now");
    let mut bytes_consumed = 0;
    let Some(input) = input else {
        return Err(CustomError::CustomErrorMessage("Empty Header List passed in".to_string()));
    };
    let mut header = Header::new();
    for item in input.split(&*SEPARATOR) {
        if item.is_empty() {
            break;
        }

        let (key, value) = Header::parse_line(&item)?;
        header.insert(key.to_string(), value.to_string());
        bytes_consumed += (item.len() + SEPARATOR.len()) as i32;
    }
    info!("Partially parsed value: {:?}", header);

    header.is_valid_token()?;

    Ok((header, bytes_consumed))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_parse_header() {
        let headers = parse_header(Some("Host: http://127.0.0.1:42062\r\nContent-Type:    text/html\r\nSet-person: this\r\nSet-person: that\r\nSet-person: now\r\n\r\n".to_string())).await.unwrap();
        println!("{:?}", headers);
        // assert_eq!(headers.get("Host"), Some(&"example.com".to_string()));
    }
}