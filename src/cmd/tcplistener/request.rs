use serde::{Serialize, Deserialize};

use tokio::{
    io::{AsyncRead, AsyncReadExt}
};
use thiserror::Error;
use once_cell::sync::Lazy;
use tracing::{
    info, error
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
pub enum ParserState {
    #[default]
    Initialized,
    Done,
    Error
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RequestLine {
    pub http_version: String, 
    pub request_target: String,
    pub method: String
}

#[derive(Debug, Default)]
pub struct ParseResponse {
    request: Request,
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

    pub fn is_done(&self) -> bool {
        self.state == ParserState::Done
    }

    pub fn is_error(&self) -> bool {
        self.state == ParserState::Error
    }

    pub async fn parse(&mut self, data: String) -> Result<i32, CustomError> {
        let mut read: i32 = 0;
        match self.state {
            ParserState::Done => {
                info!("Status is Done!");
            }
            ParserState::Error => {
                error!("Error encounted!");
                return Err(CustomError::CustomErrorMessage(format!("System in Error state")));
            }
            ParserState::Initialized => {
                let bytes_consumed = parse_request_line(data, self).await?;

                if bytes_consumed == 0 {
                    return Ok(0);
                }

                read += bytes_consumed;
            }
        }

        return Ok(read);
    }
}

async fn parse_request_line(input: String, req: &mut Request) -> Result<i32, CustomError> {
    
    let Some((http_message, _rest_of_message)) = input.split_once(&*SEPARATOR) else {
        return Ok(0);
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

    req.request_line = request_line;
    
    return Ok(input.len() as i32);
}

pub async fn request_from_reader<R>(mut io: R) -> Result<Request, CustomError> 
    where R: AsyncRead + Unpin + Send 
{
    info!("Parsing the request");
    let mut chunk = BytesMut::with_capacity(1024);
    let mut request = Request::default();

    while !&request.is_done() && !&request.is_error() {

        let len = io.read_buf(&mut chunk).await.map_err(|_| CustomError::ParseError)?;

        if len == 0 {
            continue;
        }

        let input = String::from_utf8_lossy(&chunk.to_vec()).into_owned();

        match request.parse(input).await {
            Ok(bytes_consumed) => {
                if bytes_consumed > 0 {
                    request.state = ParserState::Done;
                    info!("Successfully parsed: {:?}", request.request_line);
                    return Ok(request);
                }
            }
            Err(e) => {
                request.state = ParserState::Error;
                return Err(e);
            }
        }
        
        // request.request_line = result.request_line;

        if chunk.len() > 8192 {
            return Err(CustomError::CustomErrorMessage("Request too large".to_string()));
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