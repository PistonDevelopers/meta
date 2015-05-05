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
