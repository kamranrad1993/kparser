use std::str::FromStr;
use std::fmt;

#[derive(Debug)]
struct ParseHeadersError(String);

impl fmt::Display for ParseHeadersError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid header: {}", self.0)
    }
}

impl std::error::Error for ParseHeadersError {}

macro_rules! define_headers {
    ($($variant:ident => $name:expr),*) => {
        #[derive(Debug)]
        pub enum Headers {
            $($variant),*
        }

        impl ToString for Headers {
            fn to_string(&self) -> String {
                match self {
                    $(Headers::$variant => $name.to_string()),*
                }
            }
        }

        impl FromStr for Headers {
            type Err = ParseHeadersError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $($name => Ok(Headers::$variant),)*
                    _ => Err(ParseHeadersError(s.to_string())),
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

struct FileFieldFormData {

}

struct FormData {
    pub boundary : String,

}

pub struct Body {
    data: Vec<u8>,
}

impl Body {
    pub fn get_form_data(&self, ) -> res
}

