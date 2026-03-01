use serde::{Serialize, Deserialize};

use tokio::{
    io::{AsyncRead, AsyncReadExt}
};
use thiserror::Error;
use once_cell::sync::Lazy;
use tracing::{
    info, error, debug
};

use bytes::BytesMut;

static SEPARATOR: Lazy<String> = Lazy::new(|| String::from("\r\n"));

#[derive(Error,Debug)]
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

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
enum ParserState {
    #[default]
    Initialized,
    Done
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RequestLine {
    pub http_version: String, 
    pub request_target: String,
    pub method: String
}

#[derive(Debug, Default)]
struct ParseResponse {
    request_line: RequestLine,
    bytes_consumed: i32,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Request {
    pub request_line: RequestLine,
    pub state: ParserState,
}

impl Request {
    pub async fn new(http_version: String, request_target: String, method: String) -> Result<Request, String> {
        let request_line: RequestLine = RequestLine {
            http_version,
            request_target,
            method
        };

        let req: Request = Request {
            request_line,
            state: ParserState::Initialized
        };

        return Ok(req);
    }

    pub fn isDone(&self) -> bool {
        self.state == ParserState::Done
    }

    pub async fn parse(&mut self, data: &[u8]) -> Result<i32, CustomError> {
        let mut read: i32 = 0;
        match self.state {
            ParserState::Done => {
                tracing::info!("Status is Done!");
            }
            ParserState::Initialized => {
                let result = parse_request_line(data).await?;
                self.request_line = result.request_line;
                read += result.bytes_consumed;
            }
        }

        return Ok(read);
    }
}

async fn parse_request_line(inp: &[u8]) -> Result<ParseResponse, CustomError> {

    let input = String::from_utf8_lossy(inp);
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
        bytes_consumed: input.len() as i32
    };
    
    return Ok(result);
}

pub async fn request_from_reader<R>(mut io: R) -> Result<Request, CustomError> 
    where R: AsyncRead + Unpin + Send 
{
    info!("Parsing the request");
    let mut chunk = BytesMut::with_capacity(1024);
    let request = Request::default();

    while !&request.isDone() {

        let Ok(len) = io.read_buf(&mut chunk).await else {
            return Err(CustomError::ParseError);
        };

        if len == 0 {
            return Ok(request);
        }

    }
    return Ok(request);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request() {

    }
}