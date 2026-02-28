use serde::{Serialize, Deserialize};

use tokio::{
    io::{AsyncRead, AsyncReadExt}
};
use thiserror::Error;
use once_cell::sync::Lazy;
use tracing::{
    info, error, debug
};

static SEPARATOR: Lazy<String> = Lazy::new(|| String::from("\r\n"));

#[derive(Debug, Error)]
pub enum CustomError {
    #[error("Bad Start Line of the current HTTP Request")]    
    BadStartLine,

    #[error("Header is missing in the HTTP Reqeust")]    
    HeaderMissing,

    #[error("Malformed HTTP. Only HTTP/1.1 allowed")]    
    MalformedHttpSymbol,

    #[error("Invalid HTTP Message found")]    
    InvalidHttpMessage,

    #[error("Cannot parse the given buffer")]
    ParseError,

    #[error("Parse Error: {0}")]
    CustomErrorMessage(String),
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestLine {
    pub http_version: String, 
    pub request_target: String,
    pub method: String
}

struct ParseResponse {
    request_line: RequestLine,
    rest_of_message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub request_line: RequestLine
}

impl Request {
    pub async fn new(http_version: String, request_target: String, method: String) -> Result<Request, String> {
        let request_line: RequestLine = RequestLine {
            http_version,
            request_target,
            method
        };

        let req: Request = Request {
            request_line
        };

        return Ok(req);
    }
}

async fn parse_request_parts(input: String) -> Result<ParseResponse, CustomError> {
    let Some((http_message, rest_of_message)) = input.split_once("\r\n") else {
        return Err(CustomError::InvalidHttpMessage);
    };

    let http_split: Vec<&str> = http_message.split(" ").collect();
    if http_split.len() != 3 {
        return Err(CustomError::BadStartLine);
    }

    let Some(http_version) = http_split[2].split("/").nth(1) else {
        return Err(CustomError::InvalidHttpMessage);
    }; 

    let request_line = RequestLine {
        http_version: http_version.to_string(),
        request_target: http_split[1].to_string(),
        method: http_split[0].to_string()
    };

    let result = ParseResponse {
        request_line,
        rest_of_message: rest_of_message.to_string()
    };
    
    return Ok(result);
}

pub async fn request_from_reader<R>(mut io: R) -> Result<RequestLine, CustomError> 
    where R: AsyncRead + Unpin + Send 
{
    info!("Parsing the request");
    let mut result: String = String::new();
    let Ok(_parsed_input_bytes) = io.read_to_string(&mut result).await else {
        return Err(CustomError::ParseError);
    };
    info!("parsed string: {}", result);
    let Ok(extract_http_message) = parse_request_parts(result).await else {
        return Err(CustomError::CustomErrorMessage(String::from("Cannot parse the input buffer into parts!")));
    };

    info!("Extracted Value : {:?} ", extract_http_message.request_line);
    return Ok(extract_http_message.request_line);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request() {

    }
}