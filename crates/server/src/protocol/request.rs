use super::{END_BYTE, SEPARATOR, START_BYTE, operation::Operation};

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
