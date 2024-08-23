
#[derive(Debug)]
pub enum DataPayload{
    PadLength(Option<u16>),
    data(Vec<u8>),
    Padding(Option<u16>),
}

pub enum HeadersPayloadPriority{
    ExclusiveFlag(bool),
    StreamDependency(u32),
    Weight(u8)
}

#[derive(Debug)]
pub enum HeadersPayload{
    PadLength(Option<u16>),
    Priority(HeadersPayloadPriority),
    
    Padding(Option<u16>),
}

#[derive(Debug)]
pub enum Payload {
    DataPayload(DataPayload)
}
