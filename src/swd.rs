use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub struct Swd {
    pub version: u8,
    pub files: HashMap<u32, Rc<File>>,
}

#[derive(Debug)]
pub struct File {
    pub(crate) src: String,
    pub(crate) name: String,
    pub(crate) line_map: HashMap<u32, u32>,
    pub(crate) breakpoints: RefCell<HashMap<u32, u32>>,
}

impl File {
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
        self.line_map.get(&line).cloned()
    }

    /// Resolves a breakpoint by offset, returning the line number of the breakpoint
    pub fn resolve_breakpoint(&self, offset: u32) -> Option<u32> {
        self.breakpoints.borrow().get(&offset).cloned()
    }

    /// Adds a breakpoint by line number
    pub fn add_breakpoint(&self, line: u32) {
        if let Some(offset) = self.line_map.get(&line) {
            self.breakpoints.borrow_mut().insert(*offset, line);
        }
    }

    /// Removes a breakpoint by line number
    pub fn remove_breakpoint(&self, line: u32) {
        if let Some(offset) = self.line_map.get(&line) {
            self.breakpoints.borrow_mut().remove(offset);
        }
    }
}
