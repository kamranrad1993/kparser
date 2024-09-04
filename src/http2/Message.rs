use super::{Frame};



pub struct Message{
    settings: Frame,
    priority: Vec<Frame>,
    headers: Frame,
    data: Vec<Frame>,
    continiuation: Vec<Frame>,
    reset:Option<Frame>,
    go_awaw: Option<Frame>,
    push_promise: Vec<Frame>,
    ping: Option<Frame>, 
    window_update: Option<Frame>,
}