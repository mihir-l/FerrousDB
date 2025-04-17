#[derive(Debug)]
pub enum Operation {
    Get,
    Set,
    Delete,
}

impl Operation {
    pub(crate) fn from_bytes(bytes: &[u8]) -> Result<Operation, String> {
        match bytes {
            b"GET" => Ok(Operation::Get),
            b"SET" => Ok(Operation::Set),
            b"DEL" => Ok(Operation::Delete),
            _ => Err("Invalid operation".to_string()),
        }
    }
}
