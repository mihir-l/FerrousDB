const START_BYTE: u8 = 0x7F;
const END_BYTE: u8 = 0x7E;
const SEPARATOR: u8 = 0x1F;

#[derive(Debug)]
pub enum Operation {
    Get,
    Set,
    Delete,
}

impl Operation {
    fn from_bytes(bytes: &[u8]) -> Result<Operation, String> {
        match bytes {
            b"GET" => Ok(Operation::Get),
            b"SET" => Ok(Operation::Set),
            b"DEL" => Ok(Operation::Delete),
            _ => Err("Invalid operation".to_string()),
        }
    }
}

#[derive(Debug)]
pub struct Request {
    pub operation: Operation,
    pub key: String,
    pub value: Option<String>,
}

impl Request {
    pub fn from_bytes(bytes: &[u8]) -> Result<Request, String> {
        if bytes.len() < 3 {
            return Err("Request too short".to_string());
        }
        if bytes[0] != START_BYTE || bytes[bytes.len() - 1] != END_BYTE {
            return Err("Invalid start or end byte".to_string());
        }
        let parts = &bytes[1..bytes.len() - 1];
        let operation = Operation::from_bytes(&parts[0..=2])?;

        let package = parts[3..].split(|&b| b == SEPARATOR).collect::<Vec<_>>();

        let key = String::from_utf8_lossy(package[0]).to_string();
        let value = if package.len() > 1 {
            Some(String::from_utf8_lossy(package[1]).to_string())
        } else {
            None
        };
        Ok(Request {
            operation,
            key,
            value,
        })
    }
}

pub enum Status {
    Ok,
    Error,
}

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
