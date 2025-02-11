use uuid::Uuid;

pub fn colorize_event_type(event_type: &str) -> String {
    format!("\x1b[36m{}\x1b[0m", event_type) // Cyan for event type
}

pub fn colorize_request_id(request_id: &Uuid) -> String {
    format!("\x1b[33m{}\x1b[0m", request_id) // Yellow for request ID
}
