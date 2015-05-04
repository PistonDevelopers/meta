use {
    // Error,
    // MetaReader,
    Parameter,
    Rule,
    Token,
};

/// Stores information about whitespace.
pub struct Whitespace {
    /// Whether the whitespace is optional or required.
    pub optional: bool,
}

/*
impl MetaReader for Whitespace {
    fn start_node(&mut self, name: &str) -> Option<Error> {
        Some(Error::NotSupported)
    }
    fn end_node(&mut self) -> Option<Error> {
        Some(Error::NotSupported)
    }
    fn start_optional(&mut self) -> Option<Error> {
        Some(Error::NotSupported)
    }
    fn end_optional(&mut self) -> Option<Error>;
    fn start_select(&mut self) -> Option<Error>;
    fn reset_select(&mut self) -> Option<Error>;
    fn end_select(&mut self) -> Option<Error>;
    fn set_as_bool(&mut self, name: &str, val: bool) -> Option<Error>;
    fn set_as_str(&mut self, name: &str, val: &str) -> Option<Error>;
}
*/

/// A hard coded parameter rule for whitespace.
pub static WHITESPACE: Parameter<'static> = Parameter {
    name: "whitespace",
    args: &["optional"],
    value: None,
    body: &[
        Rule::Whitespace(Whitespace { optional: true }),
        Rule::Token(Token {
            text: "whitespace",
            inverted: None,
            predicate: None
        }),
        Rule::Select(&[
            Rule::Token(Token {
                text: "?",
                inverted: Some(false),
                predicate: Some("optional"),
            }),
            Rule::Token(Token {
                text: "!",
                inverted: Some(true),
                predicate: Some("optional"),
            }),
        ]),
    ],
};
