use crate::{
    swd::{File, Swd},
    tag::Tag,
};
use byteorder::{LittleEndian, ReadBytesExt};
use std::cell::RefCell;
use std::rc::Rc;
use std::{
    collections::HashMap,
    io::{self, ErrorKind, Read},
};

pub struct SwdReader<T> {
    stream: T,
}

impl<T: Read> SwdReader<T> {
    pub fn new(stream: T) -> Self {
        Self { stream }
    }

    pub fn read(&mut self) -> io::Result<Swd> {
        self.read_magic()?;
        let version = self.read_version()?;
        let mut files = HashMap::new();
        while let Some(tag) = self.read_tag().transpose()? {
            match tag {
                Tag::SourceFile {
                    file_index,
                    name,
                    src,
                    ..
                } => {
                    files.insert(
                        file_index,
                        Rc::new(File {
                            src,
                            name,
                            line_map: HashMap::new(),
                            breakpoints: RefCell::new(HashMap::new()),
                        }),
                    );
                }
                Tag::OffsetMap {
                    file_index,
                    line,
                    offset,
                } => {
                    if let Some(file) = files.get_mut(&file_index).and_then(Rc::get_mut) {
                        file.line_map.insert(line, offset);
                    }
                }
                Tag::SetBreakpoint { file_index, line } => {
                    if let Some(file) = files.get_mut(&(file_index as u32)).and_then(Rc::get_mut) {
                        if let Some(offset) = file.line_map.get(&(line as u32)).cloned() {
                            file.breakpoints.get_mut().insert(offset, line as u32);
                        }
                    }
                }
                // Ignore ID tag for now
                Tag::Id(_) => (),
            }
        }
        Ok(Swd { version, files })
    }

    pub fn read_magic(&mut self) -> io::Result<()> {
        let mut magic = [0; 3];
        self.stream.read_exact(&mut magic)?;
        if &magic != b"FWD" {
            return Err(io::Error::new(ErrorKind::Other, "Invalid magic"));
        }
        Ok(())
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

    pub fn read_tag(&mut self) -> Option<io::Result<Tag>> {
        let id = self.stream.read_u32::<LittleEndian>().ok()?;
        Some((|| {
            Ok(match id {
                0 => Tag::SourceFile {
                    file_index: self.stream.read_u32::<LittleEndian>()?,
                    unknown_index: self.stream.read_u32::<LittleEndian>()?,
                    name: self.read_string()?,
                    src: self.read_string()?,
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
                _ => {
                    return Err(io::Error::new(
                        ErrorKind::Other,
                        format!("Unknown tag {}", id),
                    ))
                }
            })
        })())
    }

    pub fn read_version(&mut self) -> io::Result<u8> {
        self.stream.read_u8()
    }
}
