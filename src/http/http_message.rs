use crate::Result::{self, Err, Ok};
use std::{collections::hash_map::HashMap, result, str::FromStr, vec};

use super::http::{Body, HeaderKey, HeaderValue, ParseHttpError};

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
        let spaces_index = self.split(|(&i)| i == b' ').collect::<Vec<&[u8]>>();
        if spaces_index.len() != 3 {
            return Err(ParseHttpError::InvalidHttp);
        }
        Ok(RequestStartLine {
            method: Into::<Result<RequestMethod, ParseHttpError>>::into(spaces_index[0].to_vec())?,
            path: Into::<Result<String, ParseHttpError>>::into(String::from_utf8(
                spaces_index[1].to_vec(),
            ))?,
            version: Into::<Result<String, ParseHttpError>>::into(String::from_utf8(
                spaces_index[2].to_vec(),
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
        todo!()
    }
}
impl Into<Result<HttpRequest, ParseHttpError>> for Vec<u8> {
    fn into(self) -> Result<HttpRequest, ParseHttpError> {
        todo!()
    }
}

pub struct ResponseStartLine {
    pub version: String,
    pub response_code: u32,
    pub response_msg: String,
}

pub struct HttpResponse {
    pub start_line: ResponseStartLine,
    pub headers: HashMap<HeaderKey, HeaderValue>,
    pub body: Body,
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
