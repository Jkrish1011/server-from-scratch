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
pub struct Body(String);

impl Body {

    pub fn new() -> Body {
        Body(String::new())
    }

    pub fn get(&self) -> Option<String> {
        Some(self.0.clone())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn set(&mut self, value: String) -> () {
        self.0 = value;
    }
}

pub async fn parse_body(content_length: i32, input: Option<String>) -> Result<Body, CustomError> {
    info!("Parsing the body now");
    let mut bytes_consumed = 0;
    let mut body = Body::new();

    if content_length == 0 {
        return Ok(body);
    }
    
    let Some(parsed_body) = input else {
        return Ok(body);
    };
    
    info!("parsed body value: {:?}", parsed_body.trim());
    info!("parsed body length: {:?}", parsed_body.trim().len());

    body.set(parsed_body.trim().to_string());

    Ok(body)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_parse_body() {
        let body = parse(Some("Host: http://127.0.0.1:42062\r\nContent-Type:    text/html\r\nSet-person: this\r\nSet-person: that\r\nSet-person: now\r\n\r\n".to_string())).await.unwrap();
        println!("{:?}", body);
        // assert_eq!(headers.get("Host"), Some(&"example.com".to_string()));
    }
}