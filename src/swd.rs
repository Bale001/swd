use crate::tag::Tag;

#[derive(Debug)]
pub struct Swd {
    pub version: u8,
    pub body: Vec<Tag>,
}
