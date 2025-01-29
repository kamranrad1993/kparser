use crate::Result::{self, Err, Ok};
use core::str;
use std::collections::HashMap;
use std::fmt::{self, Display};
use std::hash::Hash;
use std::str::{FromStr, Utf8Error};
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum ParseHttpError {
    InvalidHttp,
    ParseHeaderError(String),
    ParseBodyError(String),
    ParseFormDataError(String),
    UnknownString(String),
    InvalidHttpMethod,
}
impl PartialEq for ParseHttpError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::ParseHeaderError(l0), Self::ParseHeaderError(r0)) => l0 == r0,
            (Self::ParseFormDataError(l0), Self::ParseFormDataError(r0)) => l0 == r0,
            (Self::UnknownString(l0), Self::UnknownString(r0)) => l0 == r0,
            _ => false,
        }
    }
}
impl From<&str> for ParseHttpError {
    fn from(residual: &str) -> Self {
        ParseHttpError::UnknownString(String::from(residual))
    }
}
impl From<Utf8Error> for ParseHttpError {
    fn from(_residual: Utf8Error) -> Self {
        ParseHttpError::UnknownString("Invalid Utf-8".to_string())
    }
}
impl From<FromUtf8Error> for ParseHttpError {
    fn from(_residual: FromUtf8Error) -> Self {
        ParseHttpError::UnknownString("Invalid Utf-8".to_string())
    }
}
impl fmt::Display for ParseHttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid header: {}", self)
    }
}
impl std::error::Error for ParseHttpError {}
impl Into<Result<String, ParseHttpError>> for std::result::Result<String, FromUtf8Error> {
    fn into(self) -> Result<String, ParseHttpError> {
        match self {
            std::result::Result::Ok(s) => Ok(s),
            std::result::Result::Err(e) => Err(ParseHttpError::ParseBodyError(e.to_string())),
        }
    }
}

macro_rules! define_headers {
    ($($variant:ident => $name:expr),*) => {
        #[derive(Debug, PartialEq)]
        #[allow(non_camel_case_types)]
        pub enum StandardHeaders {
            $($variant),*
        }

        impl ToString for StandardHeaders {
            fn to_string(&self) -> String {
                match self {
                    $(StandardHeaders::$variant => $name.to_string()),*
                }
            }
        }

        impl FromStr for StandardHeaders {
            type Err = ParseHttpError;

            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                match s {
                    $($name => std::result::Result::Ok(StandardHeaders::$variant),)*
                    _ => std::result::Result::Err(ParseHttpError::ParseHeaderError(s.to_string())),
                }
            }
        }
    };
}

define_headers! {
    A_IM => "A-IM",
    Accept => "Accept",
    Accept_Charset => "Accept-Charset",
    Accept_Encoding => "Accept-Encoding",
    Accept_Language => "Accept-Language",
    Accept_Datetime => "Accept-Datetime",
    Access_Control_Request_Method => "Access-Control-Request-Method",
    Access_Control_Request_Headers => "Access-Control-Request-Headers",
    Authorization => "Authorization",
    Cache_Control => "Cache-Control",
    Connection => "Connection",
    Content_Disposition => "Content-Disposition",
    Content_Length => "Content-Length",
    Content_Type => "Content-Type",
    Content_Transfer_Encoding=>"Content-Transfer-Encoding",
    Cookie => "Cookie",
    Date => "Date",
    Expect => "Expect",
    Forwarded => "Forwarded",
    From => "From",
    Host => "Host",
    If_Match => "If-Match",
    If_Modified_Since => "If-Modified-Since",
    If_None_Match => "If-None-Match",
    If_Range => "If-Range",
    If_Unmodified_Since => "If-Unmodified-Since",
    Max_Forwards => "Max-Forwards",
    Origin => "Origin",
    Pragma => "Pragma",
    Proxy_Authorization => "Proxy-Authorization",
    Range => "Range",
    Referer => "Referer",
    TE => "TE",
    User_Agent => "User-Agent",
    Upgrade => "Upgrade",
    Via => "Via",
    Warning => "Warning",
    // Add additional headers here, in the same format
    Content_Security_Policy => "Content-Security-Policy",
    Strict_Transport_Security => "Strict-Transport-Security",
    X_Content_Type_Options => "X-Content-Type-Options",
    X_Frame_Options => "X-Frame-Options",
    X_XSS_Protection => "X-XSS-Protection"
    // Add the rest as required
}

pub struct CustomHeader(String);
impl PartialEq for CustomHeader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

pub enum HeaderKey {
    StandardHeader(StandardHeaders),
    CustomHeader(CustomHeader),
}
impl FromStr for HeaderKey {
    type Err = ParseHttpError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match StandardHeaders::from_str(s) {
            std::result::Result::Ok(standard_header) => {
                std::result::Result::Ok(HeaderKey::StandardHeader(standard_header))
            }
            std::result::Result::Err(_) => {
                std::result::Result::Ok(HeaderKey::CustomHeader(CustomHeader { 0: s.to_string() }))
            }
        }
    }
}
impl Display for HeaderKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HeaderKey::StandardHeader(standard_headers) => {
                f.write_str(standard_headers.to_string().as_str())
            }
            HeaderKey::CustomHeader(custom_header) => f.write_str(&custom_header.0),
        }
    }
}
impl Into<Vec<u8>> for HeaderKey {
    fn into(self) -> Vec<u8> {
        match self {
            HeaderKey::StandardHeader(standard_headers) => {
                standard_headers.to_string().as_bytes().to_vec()
            }
            HeaderKey::CustomHeader(custom_header) => custom_header.0.as_bytes().to_vec(),
        }
    }
}
impl Into<Vec<u8>> for &HeaderKey {
    fn into(self) -> Vec<u8> {
        match self {
            HeaderKey::StandardHeader(standard_headers) => {
                standard_headers.to_string().as_bytes().to_vec()
            }
            HeaderKey::CustomHeader(custom_header) => custom_header.0.as_bytes().to_vec(),
        }
    }
}
impl Into<Result<HeaderKey, ParseHttpError>> for Vec<u8> {
    fn into(self) -> Result<HeaderKey, ParseHttpError> {
        let s = match str::from_utf8(self.as_slice()) {
            std::result::Result::Ok(s) => s,
            std::result::Result::Err(_) => {
                return Err(ParseHttpError::ParseHeaderError(
                    "Invalid utf-8".to_string(),
                ))
            }
        };
        let result = match HeaderKey::from_str(s) {
            std::result::Result::Ok(result) => result,
            std::result::Result::Err(_) => unreachable!(),
        };
        Ok(result)
    }
}
impl Into<Result<HeaderKey, ParseHttpError>> for String {
    fn into(self) -> Result<HeaderKey, ParseHttpError> {
        let result = match HeaderKey::from_str(self.as_str()) {
            std::result::Result::Ok(result) => result,
            std::result::Result::Err(_) => unreachable!(),
        };
        Ok(result)
    }
}
impl Into<Result<HeaderKey, ParseHttpError>> for &[u8] {
    fn into(self) -> Result<HeaderKey, ParseHttpError> {
        Into::<Result<HeaderKey, ParseHttpError>>::into(self.to_vec())
    }
}
impl Hash for HeaderKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}
impl PartialEq for HeaderKey {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::StandardHeader(l0), Self::StandardHeader(r0)) => l0 == r0,
            (Self::CustomHeader(l0), Self::CustomHeader(r0)) => l0 == r0,
            _ => false,
        }
    }
}
impl std::cmp::Eq for HeaderKey {}

pub struct HeaderValue(String);
impl Into<Vec<u8>> for HeaderValue {
    fn into(self) -> Vec<u8> {
        self.0.as_bytes().to_vec()
    }
}
impl Into<Vec<u8>> for &HeaderValue {
    fn into(self) -> Vec<u8> {
        self.0.as_bytes().to_vec()
    }
}
impl Into<Result<HeaderValue, ParseHttpError>> for String {
    fn into(self) -> Result<HeaderValue, ParseHttpError> {
        Ok(HeaderValue { 0: self })
    }
}
impl Into<Result<HeaderValue, ParseHttpError>> for Vec<u8> {
    fn into(self) -> Result<HeaderValue, ParseHttpError> {
        match String::from_utf8(self) {
            std::result::Result::Ok(value) => Ok(HeaderValue { 0: value }),
            std::result::Result::Err(_e) => Err(ParseHttpError::ParseHeaderError(
                "Invalid utf-8 Value".to_string(),
            )),
        }
    }
}
impl Into<Result<HeaderValue, ParseHttpError>> for &[u8] {
    fn into(self) -> Result<HeaderValue, ParseHttpError> {
        Into::<Result<HeaderValue, ParseHttpError>>::into(self.to_vec())
    }
}

pub struct Header {
    pub key: HeaderKey,
    pub value: HeaderValue,
}
impl Into<Vec<u8>> for Header {
    fn into(self) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend_from_slice(self.key.to_string().as_bytes());
        result.extend_from_slice(": ".as_bytes());
        result.extend_from_slice(self.value.0.as_bytes());
        result
    }
}
impl Into<Vec<u8>> for &Header {
    fn into(self) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend_from_slice(self.key.to_string().as_bytes());
        result.extend_from_slice(": ".as_bytes());
        result.extend_from_slice(self.value.0.as_bytes());
        result
    }
}
impl Into<Result<Header, ParseHttpError>> for Vec<u8> {
    fn into(self) -> Result<Header, ParseHttpError> {
        let data_str = match String::from_utf8(self) {
            std::result::Result::Ok(result) => result,
            std::result::Result::Err(_) => {
                return Err(ParseHttpError::ParseHeaderError(
                    "Invalid utf-8".to_string(),
                ));
            }
        };

        let separate_index = match data_str.find(':') {
            Some(index) => index,
            None => {
                return Err(ParseHttpError::ParseHeaderError(format!(
                    "{}: {}",
                    "Invalid Header", data_str
                )));
            }
        };

        let key = match HeaderKey::from_str(&data_str[0..separate_index]) {
            std::result::Result::Ok(key) => key,
            std::result::Result::Err(e) => return Err(e),
        };

        let value = Into::<Result<HeaderValue, ParseHttpError>>::into(
            data_str[separate_index + 1..].to_string(),
        )?;

        Ok(Header { key, value })
    }
}
impl Into<Result<Header, ParseHttpError>> for &[u8] {
    fn into(self) -> Result<Header, ParseHttpError> {
        Into::<Result<Header, ParseHttpError>>::into(self.to_vec())
    }
}

pub struct FormDataSection {
    pub headers: HashMap<HeaderKey, HeaderValue>,
    pub data: Vec<u8>,
}
impl Into<Result<Vec<u8>, ParseHttpError>> for FormDataSection {
    fn into(self) -> Result<Vec<u8>, ParseHttpError> {
        let result = Into::<Result<Vec<u8>, ParseHttpError>>::into(&self)?;
        Ok(result)
    }
}
impl Into<Result<Vec<u8>, ParseHttpError>> for &FormDataSection {
    fn into(self) -> Result<Vec<u8>, ParseHttpError> {
        let mut result = Vec::new();
        for (key, value) in &self.headers {
            result.append(&mut Into::<Vec<u8>>::into(key));
            // result.append(&mut " ".as_bytes().to_vec());
            result.append(&mut Into::<Vec<u8>>::into(value));
            // result.append(&mut "\r\n".as_bytes().to_vec());
        }
        result.append(&mut "\r\n\r\n".as_bytes().to_vec());
        result.append(&mut self.data.clone());
        Ok(result)
    }
}

pub struct FormData {
    pub boundary: String,
    pub sections: Vec<FormDataSection>,
}
impl FormData {
    pub fn parse(boundary: String, raw_data: Vec<u8>) -> Result<FormData, ParseHttpError> {
        let boundary_marker = format!("--{}\r\n", boundary);
        let boundary_marker_bytes = boundary_marker.as_bytes();

        let mut parts = Vec::new();
        let mut start = 0;
        while let Some(pos) = raw_data[start..]
            .windows(boundary_marker.len())
            .position(|chunk| chunk == boundary_marker_bytes)
        {
            parts.push(&raw_data[start..start + pos]);
            start += pos + boundary_marker.len();
        }
        parts.push(&raw_data[start..]); // Add the remaining part

        let mut formdata_sections: Vec<FormDataSection> = Vec::new();

        for part in parts {
            if part.is_empty() || part == b"--" {
                continue; // Skip empty or terminating boundary
            }
            let body_separator = "\r\n\r\n".as_bytes();
            let sections = part
                .windows(body_separator.len())
                .position(|chunk| chunk == body_separator)
                .unwrap_or(part.len());

            let header_section = &part[0..sections];
            let body_section = &part[(sections + 4)..(part.len() - 2)];

            let mut headers: HashMap<HeaderKey, HeaderValue> = HashMap::new();
            for line in header_section.split(|&b| b == b'\n') {
                let colon_index =
                    line.iter()
                        .position(|&b| b == b':')
                        .ok_or(ParseHttpError::ParseHeaderError(
                            "Invalid formdata header".to_string(),
                        ));
                match colon_index {
                    std::result::Result::Ok(colon_index) => {
                        let (key, value) = line.split_at(colon_index);
                        headers.insert(
                            Into::<Result<HeaderKey, ParseHttpError>>::into(key)?,
                            Into::<Result<HeaderValue, ParseHttpError>>::into(value)?,
                        );
                    }
                    std::result::Result::Err(e) => return Err(e),
                }
            }

            formdata_sections.push(FormDataSection {
                headers,
                data: body_section.to_vec(),
            });
        }

        Ok(FormData {
            boundary,
            sections: formdata_sections,
        })
    }

    pub fn encode(&self) -> Result<String, ParseHttpError> {
        let s = Into::<Result<Vec<u8>, ParseHttpError>>::into(self)?;
        let l = String::from_utf8_lossy(s.as_slice());
        match l {
            std::borrow::Cow::Borrowed(v) => Ok(v.to_string()),
            std::borrow::Cow::Owned(v) => Ok(v),
        }
    }
}
impl Display for FormData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.encode() {
            Ok(value) => f.write_str(&value.as_str()),
            Err(error) => f.write_str(error.to_string().as_str()),
        }
    }
}
impl Into<Result<Vec<u8>, ParseHttpError>> for FormData {
    fn into(self) -> Result<Vec<u8>, ParseHttpError> {
        let result = Into::<Result<Vec<u8>, ParseHttpError>>::into(&self)?;
        Ok(result)
    }
}
impl Into<Result<Vec<u8>, ParseHttpError>> for &FormData {
    fn into(self) -> Result<Vec<u8>, ParseHttpError> {
        let mut result = Vec::new();
        for section in &self.sections {
            result.append(&mut "--".as_bytes().to_vec());
            result.append(&mut self.boundary.clone().as_bytes().to_vec());
            result.append(&mut "\r\n".as_bytes().to_vec());
            result.append(&mut Into::<Result<Vec<u8>, ParseHttpError>>::into(section)?.clone());
            result.append(&mut "\r\n".as_bytes().to_vec());
        }
        result.splice(result.len() - 2.., b"--".to_vec());

        Ok(result)
    }
}

pub struct Body {
    data: Vec<u8>,
}
impl Into<Result<String, ParseHttpError>> for Body {
    fn into(self) -> Result<String, ParseHttpError> {
        let result = Into::<Result<String, ParseHttpError>>::into(&self)?;
        Ok(result)
    }
}
impl Into<Result<String, ParseHttpError>> for &Body {
    fn into(self) -> Result<String, ParseHttpError> {
        Into::<Result<String, ParseHttpError>>::into(String::from_utf8(self.data.clone()))
    }
}
impl Into<Body> for String {
    fn into(self) -> Body {
        Body {
            data: self.as_bytes().to_vec(),
        }
    }
}

#[cfg(test)]
mod test_formdata {
    use super::FormData;

    #[test]
    fn form_data_test_1() {
        let boundary = "delimiter12345";
        let data = "--delimiter12345\r\nContent-Disposition: form-data; name=\"field1\"\r\n\r\nvalue1\r\n--delimiter12345\r\nContent-Disposition: form-data; name=\"field2\"; filename=\"example.txt\"\r\n\r\nvalue2\r\n--delimiter12345--";

        println!("{}", data);
        println!("{}", "++++++++++++++++++++++++++");

        let form = FormData::parse(boundary.to_string(), data.as_bytes().to_vec());
        match form {
            crate::Result::Ok(form) => {
                println!("{}", form);
                assert_eq!(form.to_string().as_str(), data);
            }
            crate::Result::Err(error) => panic!("{}", error),
        }
    }
}
