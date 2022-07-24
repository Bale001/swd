use crate::{swd::Swd, tag::Tag};
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{self, ErrorKind, Read};

pub struct SwdReader<T> {
    stream: T,
}

impl<T: Read> SwdReader<T> {
    pub fn new(stream: T) -> Self {
        Self { stream }
    }

    pub fn read(&mut self) -> io::Result<Swd> {
        let mut magic = [0; 3];
        self.stream.read_exact(&mut magic)?;
        if &magic != b"FWD" {
            return Err(io::Error::new(ErrorKind::Other, "Invalid magic"));
        }
        let version = self.read_version()?;
        let body = self.read_body()?;
        Ok(Swd { version, body })
    }

    pub fn read_string(&mut self) -> io::Result<String> {
        let mut buffer = Vec::new();
        loop {
            let b = self.stream.read_u8()?;
            if b == 0 {
                break;
            }
            buffer.push(b);
        }
        String::from_utf8(buffer).map_err(|_| io::Error::new(ErrorKind::Other, "Invalid string"))
    }

    pub fn read_body(&mut self) -> io::Result<Vec<Tag>> {
        let mut tags = Vec::new();
        loop {
            match self.read_tag() {
                Ok(tag) => tags.push(tag),
                Err(e) if e.kind() == ErrorKind::UnexpectedEof => break,
                Err(e) => return Err(e),
            }
        }
        Ok(tags)
    }

    pub fn read_tag(&mut self) -> io::Result<Tag> {
        let id = self.stream.read_u32::<LittleEndian>()?;
        Ok(match id {
            0 => Tag::SourceFile {
                file_index: self.stream.read_u32::<LittleEndian>()?,
                unknown_index: self.stream.read_u32::<LittleEndian>()?,
                file_name: self.read_string()?,
                source_code: self.read_string()?,
            },
            1 => Tag::OffsetMap {
                file_index: self.stream.read_u32::<LittleEndian>()?,
                line: self.stream.read_u32::<LittleEndian>()?,
                offset: self.stream.read_u32::<LittleEndian>()?,
            },
            2 => Tag::SetBreakpoint {
                file_index: self.stream.read_u16::<LittleEndian>()?,
                line: self.stream.read_u16::<LittleEndian>()?,
            },
            3 => {
                let mut buf = [0; 16];
                self.stream.read_exact(&mut buf)?;
                Tag::Id(buf)
            }
            _ => return Err(io::Error::new(ErrorKind::Other, "Unknown tag")),
        })
    }

    pub fn read_version(&mut self) -> io::Result<u8> {
        self.stream.read_u8()
    }
}
