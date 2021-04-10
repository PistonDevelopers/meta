/// Stores indent settings.
pub struct IndentSettings {
    /// The current indention.
    pub indent: u32,
    /// Whether to align indention to the first line.
    pub align_first: bool,
    /// The number of spaces corresponding to one tab.
    pub tab_spaces: u8,
}

impl Default for IndentSettings {
    fn default() -> IndentSettings {
        IndentSettings {
            indent: 0,
            align_first: true,
            tab_spaces: 4,
        }
    }
}
