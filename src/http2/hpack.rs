use std::collections::{HashMap, VecDeque};

use super::Len;

#[derive(Debug)]
pub enum HpackError {
    InvalidIntegerEncoding,
    InvalidStringEncoding,
    InvalidIndex,
    HuffmanDecodingError,
    HuffmanEncodingError,
}

impl From<httlib_huffman::EncoderError> for HpackError {
    fn from(value: httlib_huffman::EncoderError) -> Self {
        HpackError::HuffmanEncodingError
    }
}

impl From<httlib_huffman::DecoderError> for HpackError {
    fn from(value: httlib_huffman::DecoderError) -> Self {
        HpackError::HuffmanDecodingError
    }
}

const STATIC_TABLE: [(&str, &str); 61] = [
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

#[derive(Debug, Clone)]
pub struct HpackContext {
    dynamic_table: VecDeque<(Vec<u8>, Vec<u8>)>,
    dynamic_table_size: usize,
    max_dynamic_table_size: usize,
    name_to_index: HashMap<Vec<u8>, usize>,
}

impl HpackContext {
    pub fn new(max_size: usize) -> Self {
        let mut context = HpackContext {
            dynamic_table: VecDeque::new(),
            dynamic_table_size: 0,
            max_dynamic_table_size: max_size,
            name_to_index: HashMap::new(),
        };
        for (i, (name, _)) in STATIC_TABLE.iter().enumerate() {
            context
                .name_to_index
                .insert(name.as_bytes().to_vec(), i + 1);
        }
        context
    }

    fn add_header(&mut self, name: Vec<u8>, value: Vec<u8>) {
        let entry_size = name.len() + value.len() + 32;
        while self.dynamic_table_size + entry_size > self.max_dynamic_table_size {
            if let Some((old_name, old_value)) = self.dynamic_table.pop_back() {
                self.dynamic_table_size -= old_name.len() + old_value.len() + 32;
                self.name_to_index.remove(&old_name);
            } else {
                break;
            }
        }
        self.dynamic_table.push_front((name.clone(), value));
        self.dynamic_table_size += entry_size;
        let new_index = STATIC_TABLE.len() + self.dynamic_table.len();
        self.name_to_index.insert(name, new_index);
    }

    fn find_header(&self, name: &[u8], value: &[u8]) -> Option<usize> {
        // Check static table
        for (i, (static_name, static_value)) in STATIC_TABLE.iter().enumerate() {
            if static_name.as_bytes() == name && static_value.as_bytes() == value {
                return Some(i + 1);
            }
        }
        // Check dynamic table
        for (i, (dyn_name, dyn_value)) in self.dynamic_table.iter().enumerate() {
            if dyn_name == name && dyn_value == value {
                return Some(STATIC_TABLE.len() + i + 1);
            }
        }
        None
    }

    fn find_name(&self, name: &[u8]) -> Option<usize> {
        self.name_to_index.get(name).cloned()
    }

    pub fn resize(&mut self, new_size: usize) {
        if new_size < self.max_dynamic_table_size {
            while self.dynamic_table_size > new_size {
                if let Some((old_name, old_value)) = self.dynamic_table.pop_back() {
                    self.dynamic_table_size -= old_name.len() + old_value.len() + 32;
                    self.name_to_index.remove(&old_name);
                } else {
                    break;
                }
            }
        }
        self.max_dynamic_table_size = new_size;
    }
}

fn encode_integer(value: usize, prefix_size: u8) -> Vec<u8> {
    let mut encoded = Vec::new();
    let mask = (1 << prefix_size) - 1;
    if value < mask {
        encoded.push(value as u8);
    } else {
        encoded.push(mask as u8);
        let mut remaining = value - mask;
        while remaining >= 128 {
            encoded.push((remaining % 128 + 128) as u8);
            remaining /= 128;
        }
        encoded.push(remaining as u8);
    }
    encoded
}

fn decode_integer(data: &[u8], prefix_size: u8) -> Result<(usize, usize), HpackError> {
    let mask = (1 << prefix_size) - 1;
    let mut value = (data[0] & mask) as usize;
    if value < mask as usize {
        return Ok((value, 1));
    }

    let mut m = 0;
    for (i, &byte) in data[1..].iter().enumerate() {
        value += ((byte & 0x7f) as usize) << m;
        m += 7;
        if byte & 0x80 == 0 {
            return Ok((value, i + 2));
        }
    }
    Err(HpackError::InvalidIntegerEncoding)
}

fn encode_string(s: &[u8], use_huffman: bool) -> Result<Vec<u8>, HpackError> {
    let mut encoded = Vec::new();
    if use_huffman {
        // return Err(HpackError::HuffmanEncodingError);
        // encoded = huffman_encode(s);
        httlib_huffman::encode(s, &mut encoded)?;
    } else {
        let mut length = encode_integer(s.len(), 7);
        length[0] &= !0x80; // Clear the Huffman bit
        encoded.extend_from_slice(&length);
        encoded.extend_from_slice(s);
    }
    Ok(encoded)
}

fn decode_string(data: &[u8]) -> Result<(Vec<u8>, usize), HpackError> {
    let huffman = (data[0] & 0x80) != 0;
    let (length, mut offset) = decode_integer(data, 7)?;
    if offset + length > data.len() {
        return Err(HpackError::InvalidStringEncoding);
    }
    let string_data = &data[offset..offset + length];
    offset += length;

    let result = if huffman {
        // return Err(HpackError::HuffmanDecodingError);
        // huffman_decode(string_data).unwrap()
        let mut result = Vec::new();
        httlib_huffman::decode(
            string_data,
            &mut result,
            httlib_huffman::DecoderSpeed::FiveBits,
        )?;
        result
    } else {
        string_data.to_vec()
    };

    Ok((result, offset))
}

pub fn encode_header(name: &[u8], value: &[u8], context: &mut HpackContext) -> Vec<u8> {
    let mut encoded = Vec::new();
    if let Some(index) = context.find_header(name, value) {
        // Indexed Header Field
        encoded.extend(encode_integer(index, 7));
        encoded[0] |= 0x80;
    } else if let Some(name_index) = context.find_name(name) {
        // Literal Header Field with Incremental Indexing - Indexed Name
        encoded.extend(encode_integer(name_index, 6));
        encoded[0] |= 0x40;
        encoded.extend(encode_string(value, false).unwrap());
        context.add_header(name.to_vec(), value.to_vec());
    } else {
        // Literal Header Field with Incremental Indexing - New Name
        encoded.push(0x40);
        encoded.extend(encode_string(name, false).unwrap());
        encoded.extend(encode_string(value, false).unwrap());
        context.add_header(name.to_vec(), value.to_vec());
    }
    encoded
}

pub fn encode_headers(headers: &[(Vec<u8>, Vec<u8>)], context: &mut HpackContext) -> Vec<u8> {
    let mut encoded = Vec::new();
    for (name, value) in headers {
        encoded.extend(encode_header(name, value, context));
    }
    encoded
}

pub fn decode_headers(
    data: &[u8],
    context: &mut HpackContext,
) -> Result<(Vec<(Vec<u8>, Vec<u8>)>, usize), HpackError> {
    let mut headers = Vec::new();
    let mut offset = 0;
    let mut decompressed_size = 0;

    while offset < data.len() {
        let first_byte = data[offset];
        if first_byte & 0x80 != 0 {
            // Indexed Header Field
            let (index, consumed) = decode_integer(&data[offset..], 7)?;
            offset += consumed;
            let (name, value) = if index <= STATIC_TABLE.len() {
                let (name, value) = STATIC_TABLE[index - 1];
                (name.as_bytes().to_vec(), value.as_bytes().to_vec())
            } else {
                let dyn_index = index - STATIC_TABLE.len() - 1;
                context
                    .dynamic_table
                    .get(dyn_index)
                    .ok_or(HpackError::InvalidIndex)?
                    .clone()
            };
            decompressed_size += name.len() + value.len() + 32;
            headers.push((name, value));
        } else if first_byte & 0x40 != 0 {
            // Literal Header Field with Incremental Indexing
            let (index, consumed) = decode_integer(&data[offset..], 6)?;
            offset += consumed;
            let name = if index == 0 {
                let (name, consumed) = decode_string(&data[offset..])?;
                offset += consumed;
                name
            } else if index <= STATIC_TABLE.len() {
                STATIC_TABLE[index - 1].0.as_bytes().to_vec()
            } else {
                let dyn_index = index - STATIC_TABLE.len() - 1;
                context
                    .dynamic_table
                    .get(dyn_index)
                    .ok_or(HpackError::InvalidIndex)?
                    .0
                    .clone()
            };
            let (value, consumed) = decode_string(&data[offset..])?;
            offset += consumed;
            context.add_header(name.clone(), value.clone());
            decompressed_size += name.len() + value.len() + 32;
            headers.push((name, value));
        } else if first_byte & 0x20 != 0 {
            // Dynamic Table Size Update
            let (new_size, consumed) = decode_integer(&data[offset..], 5)?;
            offset += consumed;
            context.max_dynamic_table_size = new_size;
        } else {
            // Literal Header Field without Indexing / Never Indexed
            let (index, consumed) = decode_integer(&data[offset..], 4)?;
            offset += consumed;
            let name = if index == 0 {
                let (name, consumed) = decode_string(&data[offset..])?;
                offset += consumed;
                name
            } else if index <= STATIC_TABLE.len() {
                STATIC_TABLE[index - 1].0.as_bytes().to_vec()
            } else {
                let dyn_index = index - STATIC_TABLE.len() - 1;
                context
                    .dynamic_table
                    .get(dyn_index)
                    .ok_or(HpackError::InvalidIndex)?
                    .0
                    .clone()
            };
            let (value, consumed) = decode_string(&data[offset..])?;
            offset += consumed;
            decompressed_size += name.len() + value.len() + 32;
            headers.push((name, value));
        }
    }

    Ok((headers, decompressed_size))
}

#[derive(Debug, Clone)]
pub struct Hpack {
    encoded: Vec<u8>,
}

impl Hpack {
    pub fn new() -> Self {
        Hpack {
            encoded: Vec::new(),
        }
    }

    pub fn encode(&mut self, headers: &[(Vec<u8>, Vec<u8>)], context: &mut HpackContext) -> &[u8] {
        self.encoded = encode_headers(headers, context);
        &self.encoded
    }

    pub fn decode(
        &mut self,
        context: &mut HpackContext,
    ) -> Result<(Vec<(Vec<u8>, Vec<u8>)>, usize), HpackError> {
        decode_headers(&self.encoded, context)
    }

    pub fn encoded_size(&self) -> usize {
        self.encoded.len()
    }
}

impl From<Vec<u8>> for Hpack {
    fn from(data: Vec<u8>) -> Self {
        let mut hpack = Hpack::new(); // Default max size
        hpack.encoded = data;
        hpack
    }
}

impl From<Hpack> for Vec<u8> {
    fn from(hpack: Hpack) -> Self {
        hpack.encoded
    }
}

impl Len for Hpack {
    fn binary_len(&self) -> usize {
        self.encoded_size()
    }
}
