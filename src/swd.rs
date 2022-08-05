use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub struct Swd {
    pub version: u8,
    pub files: HashMap<u32, Rc<File>>,
    pub(crate) breakpoints: HashMap<u32, Breakpoint>,
}

#[derive(Debug, Clone)]
pub struct Breakpoint {
    pub line: u32,
    pub file: Rc<File>,
}

#[derive(Debug)]
pub struct File {
    pub(crate) id: u32,
    pub(crate) src: String,
    pub(crate) name: String,
    pub(crate) offset_map: HashMap<u32, u32>,
}

impl Swd {
    pub fn resolve_breakpoint(&self, offset: u32) -> Option<Breakpoint> {
        self.breakpoints.get(&offset).cloned()
    }

    pub fn add_breakpoint(&mut self, line: u32, file_id: u32) {
        if let Some(file) = self.files.get(&file_id).cloned() {
            if let Some(offset) = file.resolve_line(line) {
                self.breakpoints.insert(offset, Breakpoint { line, file });
            }
        }
    }

    pub fn remove_breakpoint(&mut self, line: u32, file_id: u32) {
        if let Some(file) = self.files.get(&file_id).cloned() {
            if let Some(offset) = file.resolve_line(line) {
                self.breakpoints.remove(&offset);
            }
        }
    }
}

impl File {
    /// Returns the ID of this file
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Returns the source code of this file
    pub fn src_code(&self) -> &str {
        &self.src
    }

    /// Returns the name of this file
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Resolves a line number in source code to its offset in SWF bytecode
    pub fn resolve_line(&self, line: u32) -> Option<u32> {
        self.offset_map.get(&line).cloned()
    }
}
