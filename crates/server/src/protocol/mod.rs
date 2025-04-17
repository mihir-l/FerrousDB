mod operation;
mod request;
mod response;

pub(crate) use {operation::Operation, request::Request, response::Response};

const START_BYTE: u8 = 0x7F;
const END_BYTE: u8 = 0x7E;
const SEPARATOR: u8 = 0x1F;

pub enum Status {
    Ok,
    Error,
}
