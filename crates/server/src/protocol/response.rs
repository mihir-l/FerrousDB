use super::{END_BYTE, START_BYTE, Status, operation::Operation};

pub struct Response {
    pub operation: Operation,
    pub status: Status,
    pub value: Option<String>,
}

impl Response {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![START_BYTE];
        match &self.operation {
            Operation::Get => bytes.extend_from_slice(b"GET"),
            Operation::Set => bytes.extend_from_slice(b"SET"),
            Operation::Delete => bytes.extend_from_slice(b"DEL"),
        }
        match &self.status {
            Status::Ok => bytes.extend_from_slice(b"SUCCESS"),
            Status::Error => bytes.extend_from_slice(b"FAILED"),
        }
        if let Some(value) = &self.value {
            bytes.extend_from_slice(value.as_bytes());
        }
        bytes.push(END_BYTE);
        bytes
    }
}
