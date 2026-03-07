use thiserror::Error;

#[derive(Error,Debug)]
pub enum CustomError {
    #[error("Bad Start Line of the current HTTP Request")]    
    BadStartLine,

    #[error("Header is missing in the HTTP Reqeust")]    
    HeaderMissing,

    #[error("Malformed Header Value: {0}")]
    MalformedHeader(String),

    #[error("Malformed HTTP. Only HTTP/1.1 allowed")]    
    MalformedHttpSymbol,

    #[error("Invalid HTTP Message found")]    
    InvalidHttpMessage,

    #[error("Cannot parse the given buffer")]
    ParseError,

    #[error("Parse Error: {0}")]
    CustomErrorMessage(String),
}