use std::{str::Bytes, vec};

use super::len;

const STATIC_TABLE: &[(&str, &str)] = &[
    (":authority", ""),
    (":method", "GET"),
    (":method", "POST"),
    (":path", "/"),
    (":path", "/index.html"),
    (":scheme", "http"),
    (":scheme", "https"),
    (":status", "200"),
    (":status", "204"),
    (":status", "206"),
    (":status", "304"),
    (":status", "400"),
    (":status", "404"),
    (":status", "500"),
    ("accept-charset", ""),
    ("accept-encoding", "gzip, deflate"),
    ("accept-language", ""),
    ("accept-ranges", ""),
    ("accept", ""),
    ("access-control-allow-origin", ""),
    ("age", ""),
    ("allow", ""),
    ("authorization", ""),
    ("cache-control", ""),
    ("content-disposition", ""),
    ("content-encoding", ""),
    ("content-language", ""),
    ("content-length", ""),
    ("content-location", ""),
    ("content-range", ""),
    ("content-type", ""),
    ("cookie", ""),
    ("date", ""),
    ("etag", ""),
    ("expect", ""),
    ("expires", ""),
    ("from", ""),
    ("host", ""),
    ("if-match", ""),
    ("if-modified-since", ""),
    ("if-none-match", ""),
    ("if-range", ""),
    ("if-unmodified-since", ""),
    ("last-modified", ""),
    ("link", ""),
    ("location", ""),
    ("max-forwards", ""),
    ("proxy-authenticate", ""),
    ("proxy-authorization", ""),
    ("range", ""),
    ("referer", ""),
    ("refresh", ""),
    ("retry-after", ""),
    ("server", ""),
    ("set-cookie", ""),
    ("strict-transport-security", ""),
    ("transfer-encoding", ""),
    ("user-agent", ""),
    ("vary", ""),
    ("via", ""),
    ("www-authenticate", ""),
];

#[derive(Debug)]
pub struct HpackHeaders {
    headers: Vec<(String, String)>,
}

impl HpackHeaders {
    pub fn new() -> Self {
        Self {
            headers: Vec::new(),
        }
    }

    pub fn add_header(&mut self, key: String, value: String) {
        self.headers.push((key, value));
    }

    pub fn get_headers(&self) -> &Vec<(String, String)> {
        self.headers.as_ref()
    }

    fn encode_header(name: &str, value: &str) -> Vec<u8> {
        let mut encoded = Vec::new();

        // Try to find the header field in the static table
        if let Some((index, _)) = STATIC_TABLE
            .iter()
            .enumerate()
            .find(|(_, &(n, v))| n == name && (v.is_empty() || v == value))
        {
            // Indexed Header Field Representation
            encoded.push(0b10000000 | (index + 1) as u8);
        } else {
            // Literal Header Field with Incremental Indexing
            encoded.push(0b01000000);
            encoded.push(name.len() as u8);
            encoded.extend_from_slice(name.as_bytes());
            encoded.push(value.len() as u8);
            encoded.extend_from_slice(value.as_bytes());
        }

        encoded
    }

    fn encode_headers(headers: &[(String, String)]) -> Vec<u8> {
        let mut encoded_payload = Vec::new();

        for (name, value) in headers {
            let encoded_header = HpackHeaders::encode_header(name, value);
            encoded_payload.extend_from_slice(&encoded_header);
        }

        encoded_payload
    }

    fn decode_single_header(encoded: &[u8]) -> Option<((String, String), usize)> {
        if encoded.is_empty() {
            return None;
        }

        let first_byte = encoded[0];
        if first_byte & 0b10000000 == 0b10000000 {
            // Indexed Header Field Representation
            let index = (first_byte & 0b01111111) as usize - 1;
            if index < STATIC_TABLE.len() {
                let (name, value) = STATIC_TABLE[index];
                return Some(((name.to_string(), value.to_string()), 1));
            }
        } else if first_byte & 0b01000000 == 0b01000000 {
            // Literal Header Field with Incremental Indexing
            let name_len = (encoded[1] & 0b01111111) as usize;
            let l1=encoded[2..(2 + name_len)].to_vec();
            let name = String::from_utf8_lossy(&encoded[2..(2 + name_len)]).to_string();
            let value_len = (encoded[2 + name_len] & 0b01111111) as usize;
            let l2 = encoded[2+ name_len];
            let value = String::from_utf8_lossy(&encoded[(3 + name_len)..(3 + name_len + value_len)])
                .to_string();
            return Some(((name, value), 3 + name_len + value_len));
        }

        None
    }

    fn decode_headers(encoded_payload: &[u8]) -> Vec<(String, String)> {
        let mut headers = Vec::new();
        let mut pos = 0;

        while pos < encoded_payload.len() {
            if let Some((header, header_len)) =
                HpackHeaders::decode_single_header(&encoded_payload[pos..])
            {
                headers.push(header);
                pos += header_len;
            } else {
                break;
            }
        }

        headers
    }
}

impl Into<Vec<u8>> for HpackHeaders {
    fn into(self) -> Vec<u8> {
        HpackHeaders::encode_headers(&self.headers)
    }
}

impl From<Vec<u8>> for HpackHeaders {
    fn from(value: Vec<u8>) -> Self {
        let headers = HpackHeaders::decode_headers(&value);
        Self { headers: headers }
    }
}

impl len for HpackHeaders {
    fn binary_len(&self)->usize {
        HpackHeaders::encode_headers(&self.headers).len() // TODO : improve 
    }
}