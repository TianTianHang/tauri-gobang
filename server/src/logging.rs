use tracing::{span, Level};
use uuid::Uuid;

pub fn generate_request_id() -> String {
    Uuid::new_v4().to_string()
}

pub fn request_id_span(request_id: &str) -> span::Span {
    span!(Level::DEBUG, "request", request_id = %request_id)
}

pub fn connection_id_span(connection_id: &str) -> span::Span {
    span!(Level::DEBUG, "connection", connection_id = %connection_id)
}

pub fn room_span(room_id: &str) -> span::Span {
    span!(Level::DEBUG, "room", room_id = %room_id)
}
