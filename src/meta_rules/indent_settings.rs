/// Stores indent settings.
pub struct IndentSettings {
    /// The current indention.
    pub ident: u32,
}

impl Default for IndentSettings {
    fn default() -> IndentSettings {IndentSettings {ident: 0}}
}
