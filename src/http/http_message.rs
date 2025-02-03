use crate::Result::{self, Err, Ok};
use std::{collections::hash_map::HashMap, result, str::FromStr, vec};

use super::http::{Body, Header, HeaderKey, HeaderValue, ParseHttpError};

pub const VERSION: &str = "HTTP/1.1";
pub enum RequestMethod {
    CONNECT,
    DELETE,
    GET,
    HEAD,
    OPTIONS,
    PATCH,
    POST,
    PUT,
    TRACE,
}
impl Into<Result<Vec<u8>, ParseHttpError>> for RequestMethod {
    fn into(self) -> Result<Vec<u8>, ParseHttpError> {
        match self {
            RequestMethod::CONNECT => Ok("CONNECT".as_bytes().to_vec()),
            RequestMethod::DELETE => Ok("DELETE".as_bytes().to_vec()),
            RequestMethod::GET => Ok("GET".as_bytes().to_vec()),
            RequestMethod::HEAD => Ok("HEAD".as_bytes().to_vec()),
            RequestMethod::OPTIONS => Ok("OPTIONS".as_bytes().to_vec()),
            RequestMethod::PATCH => Ok("PATCH".as_bytes().to_vec()),
            RequestMethod::POST => Ok("POST".as_bytes().to_vec()),
            RequestMethod::PUT => Ok("PUT".as_bytes().to_vec()),
            RequestMethod::TRACE => Ok("TRACE".as_bytes().to_vec()),
            _ => Err(ParseHttpError::InvalidHttpMethod),
        }
    }
}
impl Into<Result<RequestMethod, ParseHttpError>> for Vec<u8> {
    fn into(self) -> Result<RequestMethod, ParseHttpError> {
        let data_str = Into::<Result<String, ParseHttpError>>::into(String::from_utf8(self))?;
        match data_str.as_str() {
            "CONNECT" => Ok(RequestMethod::CONNECT),
            "DELETE" => Ok(RequestMethod::DELETE),
            "GET" => Ok(RequestMethod::GET),
            "HEAD" => Ok(RequestMethod::HEAD),
            "OPTIONS" => Ok(RequestMethod::OPTIONS),
            "PATCH" => Ok(RequestMethod::PATCH),
            "POST" => Ok(RequestMethod::POST),
            "PUT" => Ok(RequestMethod::PUT),
            "TRACE" => Ok(RequestMethod::TRACE),
            _ => Err(ParseHttpError::InvalidHttpMethod),
        }
    }
}

pub struct RequestStartLine {
    pub method: RequestMethod,
    pub path: String,
    version: String,
}
impl Into<Result<Vec<u8>, ParseHttpError>> for RequestStartLine {
    fn into(self) -> Result<Vec<u8>, ParseHttpError> {
        let mut result = Vec::new();
        result.append(&mut Into::<Result<Vec<u8>, ParseHttpError>>::into(
            self.method,
        )?);
        result.append(&mut " ".as_bytes().to_vec());
        result.append(&mut self.path.as_bytes().to_vec());
        result.append(&mut " ".as_bytes().to_vec());
        result.append(&mut VERSION.as_bytes().to_vec());

        Ok(result)
    }
}
impl Into<Result<RequestStartLine, ParseHttpError>> for Vec<u8> {
    fn into(self) -> Result<RequestStartLine, ParseHttpError> {
        let splitted = self.split(|(&i)| i == b' ').collect::<Vec<&[u8]>>();
        if splitted.len() != 3 {
            return Err(ParseHttpError::InvalidHttp);
        }
        Ok(RequestStartLine {
            method: Into::<Result<RequestMethod, ParseHttpError>>::into(splitted[0].to_vec())?,
            path: Into::<Result<String, ParseHttpError>>::into(String::from_utf8(
                splitted[1].to_vec(),
            ))?,
            version: Into::<Result<String, ParseHttpError>>::into(String::from_utf8(
                splitted[2].to_vec(),
            ))?,
        })
    }
}

pub struct HttpRequest {
    pub start_line: RequestStartLine,
    pub headers: HashMap<HeaderKey, HeaderValue>,
    pub body: Body,
}
impl Into<Result<Vec<u8>, ParseHttpError>> for HttpRequest {
    fn into(self) -> Result<Vec<u8>, ParseHttpError> {
        let mut result = Vec::new();

        result.append(&mut Into::<Result<Vec<u8>, ParseHttpError>>::into(
            self.start_line,
        )?);
        result.append(&mut "\r\n".as_bytes().to_vec());

        for (key, value) in self.headers {
            result.append(&mut key.into());
            result.append(&mut ": ".as_bytes().to_vec());
            result.append(&mut value.into());
            result.append(&mut "\r\n".as_bytes().to_vec());
        }
        result.append(&mut "\r\n".as_bytes().to_vec());

        result.append(&mut Into::<Result<Vec<u8>, ParseHttpError>>::into(
            self.body,
        )?);

        Ok(result)
    }
}
impl Into<Result<HttpRequest, ParseHttpError>> for Vec<u8> {
    fn into(self) -> Result<HttpRequest, ParseHttpError> {
        let mut start = 0;
        let mut lines = Vec::new();
        while let Some(pos) = self[start..]
            .windows(2)
            .position(|line_break| line_break == b"\r\n")
        {
            lines.push(&self[start..start + pos]);
            start += pos + 2;
        }
        let mut lines = lines.iter().peekable();

        let start_line = match lines.next() {
            Some(line) => Into::<Result<RequestStartLine, ParseHttpError>>::into(line.to_vec())?,
            None => return Err(ParseHttpError::InvalidHttp),
        };

        let mut headers = HashMap::new();
        while let Some(line) = lines.peek() {
            if line.is_empty() {
                lines.next();
                break;
            }
            let header = Into::<Result<Header, ParseHttpError>>::into(line.to_vec())?;
            headers.insert(header.key, header.value);
            lines.next();
        }

        let body = lines.flat_map(|line| line.to_vec()).collect();

        Ok(HttpRequest {
            start_line,
            headers,
            body: Body::Data(body),
        })
    }
}

pub struct ResponseStartLine {
    pub version: String,
    pub response_code: u32,
    pub response_msg: String,
}
impl Into<Result<Vec<u8>, ParseHttpError>> for ResponseStartLine {
    fn into(self) -> Result<Vec<u8>, ParseHttpError> {
        let mut result = Vec::new();
        result.append(&mut self.version.as_bytes().to_vec());
        result.append(&mut " ".as_bytes().to_vec());
        result.append(&mut self.response_code.to_string().as_bytes().to_vec());
        result.append(&mut " ".as_bytes().to_vec());
        result.append(&mut self.response_msg.as_bytes().to_vec());
        // result.append(&mut "\r\n".as_bytes().to_vec());

        Ok(result)
    }
}
impl Into<Result<ResponseStartLine, ParseHttpError>> for Vec<u8> {
    fn into(self) -> Result<ResponseStartLine, ParseHttpError> {
        let splitted = self.split(|(&i)| i == b' ').collect::<Vec<&[u8]>>();
        if splitted.len() < 3 {
            return Err(ParseHttpError::InvalidHttp);
        }
        let version =
            Into::<Result<String, ParseHttpError>>::into(String::from_utf8(splitted[0].to_vec()))?;
        let response_code =
            Into::<Result<String, ParseHttpError>>::into(String::from_utf8(splitted[1].to_vec()))?
                .parse::<u32>()
                .unwrap();
        let response_msg = Into::<Result<String, ParseHttpError>>::into(String::from_utf8(
            splitted[2..].join(&b' '),
        ))?;

        Ok(ResponseStartLine {
            version,
            response_code,
            response_msg,
        })
    }
}

pub struct HttpResponse {
    pub start_line: ResponseStartLine,
    pub headers: HashMap<HeaderKey, HeaderValue>,
    pub body: Body,
}
impl Into<Result<Vec<u8>, ParseHttpError>> for HttpResponse {
    fn into(self) -> Result<Vec<u8>, ParseHttpError> {
        let mut result = Vec::new();

        result.append(&mut Into::<Result<Vec<u8>, ParseHttpError>>::into(
            self.start_line,
        )?);
        result.append(&mut "\r\n".as_bytes().to_vec());

        for (key, value) in self.headers {
            result.append(&mut key.into());
            result.append(&mut ": ".as_bytes().to_vec());
            result.append(&mut value.into());
            result.append(&mut "\r\n".as_bytes().to_vec());
        }
        result.append(&mut "\r\n".as_bytes().to_vec());

        result.append(&mut Into::<Result<Vec<u8>, ParseHttpError>>::into(
            self.body,
        )?);

        Ok(result)
    }
}
impl Into<Result<HttpResponse, ParseHttpError>> for Vec<u8> {
    fn into(self) -> Result<HttpResponse, ParseHttpError> {
        let mut start = 0;
        let mut lines = Vec::new();
        while let Some(pos) = self[start..]
            .windows(2)
            .position(|line_break| line_break == b"\r\n")
        {
            lines.push(&self[start..start + pos]);
            start += pos + 2;
        }
        let mut lines = lines.iter().peekable();

        let start_line = match lines.next() {
            Some(line) => Into::<Result<ResponseStartLine, ParseHttpError>>::into(line.to_vec())?,
            None => return Err(ParseHttpError::InvalidHttp),
        };

        let mut headers = HashMap::new();
        while let Some(line) = lines.peek() {
            if line.is_empty() {
                lines.next();
                break;
            }
            let header = Into::<Result<Header, ParseHttpError>>::into(line.to_vec())?;
            headers.insert(header.key, header.value);
            lines.next();
        }

        let body = lines.flat_map(|line| line.to_vec()).collect();

        Ok(HttpResponse {
            start_line,
            headers,
            body: Body::Data(body),
        })
    }
}

pub enum HttpMessage {
    Request(HttpRequest),
    Response(HttpResponse),
}
impl Into<Result<Vec<u8>, ParseHttpError>> for HttpMessage {
    fn into(self) -> Result<Vec<u8>, ParseHttpError> {
        todo!()
    }
}
impl Into<Result<HttpMessage, ParseHttpError>> for Vec<u8> {
    fn into(self) -> Result<HttpMessage, ParseHttpError> {
        todo!()
    }
}

#[cfg(test)]
mod test_http {
    use super::{Body, HttpRequest, RequestMethod, RequestStartLine, VERSION};
    use crate::http::http::{Header, HeaderKey, HeaderValue, ParseHttpError};
    use crate::http::http_message::{HttpResponse, ResponseStartLine};
    use crate::Result;
    use std::collections::HashMap;

    #[test]
    fn http_request_test() {
        // Test request serialization
        let request = HttpRequest {
            start_line: RequestStartLine {
                method: RequestMethod::GET,
                path: String::from("/index.html"),
                version: String::from(VERSION),
            },
            headers: {
                let mut headers = HashMap::new();
                headers.insert(
                    HeaderKey::new("Host".to_string()),
                    HeaderValue::new("example.com".to_string()),
                );
                headers
            },
            body: Body::Data(vec![]),
        };

        let bytes: Vec<u8> = Into::<Result<Vec<u8>, ParseHttpError>>::into(request).unwrap();
        let expected = "GET /index.html HTTP/1.1\r\nHost: example.com\r\n\r\n";
        assert_eq!(String::from_utf8(bytes).unwrap(), expected);

        // Test request parsing
        let input = b"GET /test.html HTTP/1.1\r\nContent-Type: text/html\r\n\r\nHello World";
        let request: HttpRequest =
            Into::<Result<HttpRequest, ParseHttpError>>::into(input.to_vec()).unwrap();

        assert!(matches!(request.start_line.method, RequestMethod::GET));
        assert_eq!(request.start_line.path, "/test.html");
        // assert_eq!(
        //     request
        //         .headers
        //         .get(&HeaderKey::from("Content-Type".to_string()))
        //         .unwrap(),
        //     &HeaderValue::from("text/html".to_string())
        // );
        // assert_eq!(
        //     Into::<Vec<u8>>::into(request.body).unwrap(),
        //     b"Hello World".to_vec()
        // );
    }

    #[test]
    #[should_panic]
    fn invalid_request_test() {
        // Test invalid request parsing
        let input = b"INVALID /test.html HTTP/1.1\r\n\r\n";
        let _: HttpRequest = Into::<Result<HttpRequest, _>>::into(input.to_vec()).unwrap();
    }

    #[test]
    fn http_response_test() {
        // Test response serialization
        let response = HttpResponse {
            start_line: ResponseStartLine {
                version: String::from(VERSION),
                response_code: 200,
                response_msg: String::from("OK"),
            },
            headers: {
                let mut headers = HashMap::new();
                headers.insert(
                    HeaderKey::new("Content-Type".to_string()),
                    HeaderValue::new("text/html".to_string()),
                );
                headers
            },
            body: Body::Data(b"Hello World".to_vec()),
        };

        let bytes: Vec<u8> = Into::<Result<Vec<u8>, ParseHttpError>>::into(response).unwrap();
        let expected = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\nHello World";
        assert_eq!(String::from_utf8(bytes).unwrap(), expected);

        // Test response parsing
        let input = b"HTTP/1.1 404 Not Found\r\nContent-Type: text/html\r\n\r\nPage not found";
        let response: HttpResponse =
            Into::<Result<HttpResponse, ParseHttpError>>::into(input.to_vec()).unwrap();

        assert_eq!(response.start_line.version, "HTTP/1.1");
        assert_eq!(response.start_line.response_code, 404);
        assert_eq!(response.start_line.response_msg, "Not Found");
        // assert_eq!(
        //     response
        //         .headers
        //         .get(&HeaderKey::from("Content-Type".to_string()))
        //         .unwrap(),
        //     &HeaderValue::from("text/html".to_string())
        // );
        // assert_eq!(
        //     Into::<Vec<u8>>::into(response.body).unwrap(),
        //     b"Page not found".to_vec()
        // );
    }
}
