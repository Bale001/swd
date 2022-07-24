#[derive(Debug)]
pub enum Tag {
    SourceFile {
        file_index: u32,
        unknown_index: u32,
        file_name: String,
        source_code: String,
    },
    OffsetMap {
        file_index: u32,
        line: u32,
        offset: u32,
    },
    SetBreakpoint {
        file_index: u16,
        line: u16,
    },
    Id([u8; 16]),
}
