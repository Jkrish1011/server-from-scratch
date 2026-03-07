use serde::{Serialize, Deserialize};

use tokio::{
    io::{AsyncRead, AsyncReadExt}
};
use thiserror::Error;
use once_cell::sync::Lazy;
use tracing::{
    info, error
};

use bytes::{
    BytesMut,
    Buf
};

use crate::errors::CustomError;
use crate::headers;

static SEPARATOR: Lazy<String> = Lazy::new(|| String::from("\r\n"));



#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ParserState {
    #[default]
    Initialized,
    Headers,
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
        info!("Is Done is called. result: {}", self.state == ParserState::Done);
        self.state == ParserState::Done
    }

    pub fn is_error(&self) -> bool {
        info!("Is Error is called. result: {}", self.state == ParserState::Error);
        self.state == ParserState::Error
    }

    pub async fn parse(&mut self, data: String) -> Result<i32, CustomError> {
        let mut read: i32 = 0;
        match self.state {
            ParserState::Done => {
                info!("Status is Done!");
            }
            ParserState::Headers => {
                info!("Processing Headers");
                let (headers, bytes_consumed) = headers::parse(Some(data)).await?;
                info!("headers:: {:?}", headers);
                self.state = ParserState::Done;
                read += bytes_consumed;
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
                info!("bytes consumed is : {}", bytes_consumed);
                read += bytes_consumed;
                self.state = ParserState::Headers;
                info!("State is now transition to Headers");
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
    
    return Ok(http_message.len() as i32 + 2);
}

pub async fn request_from_reader<R>(mut io: R) -> Result<Request, CustomError> 
    where R: AsyncRead + Unpin + Send 
{
    info!("Parsing the request");
    let mut chunk = BytesMut::with_capacity(1024);
    let mut request = Request::default();

    let len = io.read_buf(&mut chunk).await.map_err(|_| CustomError::ParseError)?;

    if len == 0 {
        return Ok(request);
    }

    while !&request.is_done() && !&request.is_error() {
        info!("In the loop!");
        info!("Request state is : {:?}", request.state);
    
        let input = String::from_utf8_lossy(&chunk.to_vec()).into_owned();
        info!("This is the actual input: {}", input);

        match request.parse(input).await {
            Ok(bytes_consumed) => {
                if bytes_consumed > 0 {
                    info!("Successfully parsed: {:?} by consuming {} bytes", request.request_line, bytes_consumed);

                    chunk.advance(bytes_consumed as usize);

                    if request.is_done() {
                        return Ok(request);
                    }
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