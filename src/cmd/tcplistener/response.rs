pub struct ResponseObject {
    pub ok: i32,
    pub unauthorized: i32,
    pub server_error: i32,
    pub unknown_error: i32
}

impl ResponseObject {
    pub fn new() -> Self {
        Self {
            ok: 200,
            unauthorized: 400,
            server_error: 500,
            unknown_error: 501
        }
    }

    pub fn get_reply_status(&self, code: i32, body: &[u8]) -> String {
        let total_length =  body.len();
        match code {
            200 => {
                let response = format!("HTTP/1.1 200 OK\r\nContent-Type: application/text\r\nContent-Length: {}\r\n\r\n", total_length);
                return response;
            },
            400 => {
                let response = format!("HTTP/1.1 400 Bad Reqyest\r\nContent-Type: application/text\r\nContent-Length: {}\r\n\r\n", total_length);
                return response;
            },
            500 => {
                let response = format!("HTTP/1.1 500 Internal Server Error\r\nContent-Type: application/text\r\nContent-Length: {}\r\n\r\n", total_length);
                return response;
            },
            _ => {
                let response = format!("HTTP/1.1 501 Uknown Server Error\r\nContent-Type: application/text\r\nContent-Length: {}\r\n\r\n", total_length);
                return response;
            },
        }
    }
}