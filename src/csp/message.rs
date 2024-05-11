#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message<T, R> {
    pub data: T,
    pub responder: Option<R>,
}

impl<T, R> Message<T, R> {
    /// Creates message which asks for response
    pub const fn new(data: T, responder: R) -> Self {
        Self {
            data,
            responder: Some(responder),
        }
    }

    /// Creates message without need for responding
    pub const fn without_responder(data: T) -> Self {
        Self {
            data,
            responder: None,
        }
    }

    pub const fn needs_response(&self) -> bool {
        self.responder.is_some()
    }
}
