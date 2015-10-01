use std::sync::Arc;
use std::cell::Cell;

use meta_rules::{
    update_refs,
    Lines,
    Node,
    Number,
    Optional,
    Rule,
    Select,
    SeparateBy,
    Sequence,
    Text,
    Token,
    UntilAnyOrWhitespace,
    Whitespace,
};
use Syntax;

/// Returns rules for parsing meta rules.
pub fn rules() -> Syntax {
    let opt: Arc<String> = Arc::new("optional".into());
    let inv: Arc<String> = Arc::new("inverted".into());
    let prop: Arc<String> = Arc::new("property".into());
    let any: Arc<String> = Arc::new("any_characters".into());
    let seps: Arc<String> = Arc::new("[]{}():.!?\"".into());

    // 1 "string" [w? ..seps!"name" ":" w? t?"text"]
    let string_rule = Rule::Sequence(Sequence {
        debug_id: 1001,
        args: vec![
            Rule::Whitespace(Whitespace {
                debug_id: 1002,
                optional: true,
            }),
            Rule::UntilAnyOrWhitespace(UntilAnyOrWhitespace {
                debug_id: 1003,
                any_characters: seps.clone(),
                optional: false,
                property: Some(Arc::new("name".into()))
            }),
            Rule::Token(Token {
                debug_id: 1004,
                text: Arc::new(":".into()),
                not: false,
                inverted: false,
                property: None
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 1005,
                optional: true,
            }),
            Rule::Text(Text {
                debug_id: 1006,
                allow_empty: true,
                property: Some(Arc::new("text".into())),
            })
        ]
    });

    // 2 "node" [w? $"id" w! t!"name" w! @"rule""rule"]
    let node_rule = Rule::Sequence(Sequence {
        debug_id: 2001,
        args: vec![
            Rule::Whitespace(Whitespace {
                debug_id: 2002,
                optional: true,
            }),
            Rule::Number(Number {
                debug_id: 2003,
                allow_underscore: false,
                property: Some(Arc::new("id".into())),
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 2004,
                optional: false,
            }),
            Rule::Text(Text {
                debug_id: 2005,
                allow_empty: false,
                property: Some(Arc::new("name".into())),
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 2006,
                optional: false,
            }),
            Rule::Node(Node {
                debug_id: 2007,
                name: Arc::new("rule".into()),
                property: Some(Arc::new("rule".into())),
                index: Cell::new(None),
            })
        ]
    });

    // 3 "set" {t!"value" ..seps!"ref"}
    let set_rule = Rule::Select(Select {
        debug_id: 3001,
        args: vec![
            Rule::Text(Text {
                debug_id: 3002,
                allow_empty: false,
                property: Some(Arc::new("value".into())),
            }),
            Rule::UntilAnyOrWhitespace(UntilAnyOrWhitespace {
                debug_id: 3003,
                any_characters: seps.clone(),
                optional: false,
                property: Some(Arc::new("ref".into())),
            })
        ]
    });

    // 4 "set_opt" {t?"value" ..seps!"ref"}
    let set_opt_rule = Rule::Select(Select {
        debug_id: 4001,
        args: vec![
            Rule::Text(Text {
                debug_id: 4002,
                allow_empty: true,
                property: Some(Arc::new("value".into())),
            }),
            Rule::UntilAnyOrWhitespace(UntilAnyOrWhitespace {
                debug_id: 4003,
                any_characters: seps.clone(),
                optional: false,
                property: Some(Arc::new("ref".into())),
            })
        ]
    });

    // 5 "opt" {"?"opt "!"!opt}
    let opt_rule = Rule::Select(Select {
        debug_id: 5001,
        args: vec![
            Rule::Token(Token {
                debug_id: 5002,
                text: Arc::new("?".into()),
                not: false,
                inverted: false,
                property: Some(opt.clone())
            }),
            Rule::Token(Token {
                debug_id: 5003,
                text: Arc::new("!".into()),
                not: false,
                inverted: true,
                property: Some(opt.clone())
            }),
        ]
    });

    // 6 "number" ["$" ?"_""underscore" ?@"set"prop]
    let number_rule = Rule::Sequence(Sequence {
        debug_id: 6001,
        args: vec![
            Rule::Token(Token {
                debug_id: 6002,
                text: Arc::new("$".into()),
                not: false,
                inverted: false,
                property: None,
            }),
            Rule::Optional(Box::new(Optional {
                debug_id: 6003,
                rule: Rule::Token(Token {
                    debug_id: 6004,
                    text: Arc::new("_".into()),
                    not: false,
                    inverted: false,
                    property: Some(Arc::new("underscore".into()))
                })
            })),
            Rule::Optional(Box::new(Optional {
                debug_id: 6005,
                rule: Rule::Node(Node {
                    debug_id: 6006,
                    name: Arc::new("set".into()),
                    property: Some(Arc::new("property".into())),
                    index: Cell::new(None),
                })
            }))
        ]
    });

    // 7 "text" ["t" {"?""allow_empty" "!"!"allow_empty"} ?@"set"prop]
    let text_rule = Rule::Sequence(Sequence {
        debug_id: 7001,
        args: vec![
            Rule::Token(Token {
                debug_id: 7002,
                text: Arc::new("t".into()),
                not: false,
                inverted: false,
                property: None,
            }),
            Rule::Select(Select {
                debug_id: 7003,
                args: vec![
                    Rule::Token(Token {
                        debug_id: 7004,
                        text: Arc::new("?".into()),
                        not: false,
                        inverted: false,
                        property: Some(Arc::new("allow_empty".into())),
                    }),
                    Rule::Token(Token {
                        debug_id: 7005,
                        text: Arc::new("!".into()),
                        not: false,
                        inverted: true,
                        property: Some(Arc::new("allow_empty".into())),
                    })
                ]
            }),
            Rule::Optional(Box::new(Optional {
                debug_id: 7006,
                rule: Rule::Node(Node {
                    debug_id: 7007,
                    name: Arc::new("set".into()),
                    property: Some(prop.clone()),
                    index: Cell::new(None),
                })
            })),
        ]
    });

    // 8 "reference" ["@" t!"name" ?@"set"prop]
    let reference_rule = Rule::Sequence(Sequence {
        debug_id: 8001,
        args: vec![
            Rule::Token(Token {
                debug_id: 8002,
                text: Arc::new("@".into()),
                not: false,
                inverted: false,
                property: None,
            }),
            Rule::Text(Text {
                debug_id: 8003,
                allow_empty: false,
                property: Some(Arc::new("name".into())),
            }),
            Rule::Optional(Box::new(Optional {
                debug_id: 8004,
                rule: Rule::Node(Node {
                    debug_id: 8005,
                    name: Arc::new("set".into()),
                    property: Some(prop.clone()),
                    index: Cell::new(None)
                })
            }))
        ]
    });

    // 9 "sequence" ["[" w? s!.(w!) {@"rule""rule"} "]"]
    let sequence_rule = Rule::Sequence(Sequence {
        debug_id: 9001,
        args: vec![
            Rule::Token(Token {
                debug_id: 9002,
                text: Arc::new("[".into()),
                not: false,
                inverted: false,
                property: None,
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 9003,
                optional: true,
            }),
            Rule::SeparateBy(Box::new(SeparateBy {
                debug_id: 9004,
                optional: false,
                allow_trail: true,
                by: Rule::Whitespace(Whitespace {
                    debug_id: 9005,
                    optional: false,
                }),
                rule: Rule::Node(Node {
                    debug_id: 9006,
                    name: Arc::new("rule".into()),
                    property: Some(Arc::new("rule".into())),
                    index: Cell::new(None)
                })
            })),
            Rule::Token(Token {
                debug_id: 9007,
                text: Arc::new("]".into()),
                not: false,
                inverted: false,
                property: None,
            })
        ]
    });

    // 10 "select" ["{" w? s!.(w!) {@"rule""rule"} "}"]
    let select_rule = Rule::Sequence(Sequence {
        debug_id: 10001,
        args: vec![
            Rule::Token(Token {
                debug_id: 10002,
                text: Arc::new("{".into()),
                not: false,
                inverted: false,
                property: None,
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 10003,
                optional: true,
            }),
            Rule::SeparateBy(Box::new(SeparateBy {
                debug_id: 10004,
                optional: false,
                allow_trail: true,
                by: Rule::Whitespace(Whitespace {
                    debug_id: 10005,
                    optional: false,
                }),
                rule: Rule::Node(Node {
                    debug_id: 10006,
                    name: Arc::new("rule".into()),
                    property: Some(Arc::new("rule".into())),
                    index: Cell::new(None),
                })
            })),
            Rule::Token(Token {
                debug_id: 10007,
                text: Arc::new("}".into()),
                not: false,
                inverted: false,
                property: None,
            })
        ]
    });

    // 11 "separated_by" ["s" @"opt" ?".""allow_trail"
    //  "(" w? @"rule""by" w? ")" w? "{" w? @"rule""rule" w? "}"]
    let separated_by_rule = Rule::Sequence(Sequence {
        debug_id: 11001,
        args: vec![
            Rule::Token(Token {
                debug_id: 11002,
                text: Arc::new("s".into()),
                not: false,
                inverted: false,
                property: None,
            }),
            Rule::Node(Node {
                debug_id: 11003,
                name: Arc::new("opt".into()),
                property: None,
                index: Cell::new(None),
            }),
            Rule::Optional(Box::new(Optional {
                debug_id: 11004,
                rule: Rule::Token(Token {
                    debug_id: 11005,
                    text: Arc::new(".".into()),
                    not: false,
                    inverted: false,
                    property: Some(Arc::new("allow_trail".into())),
                })
            })),
            Rule::Token(Token {
                debug_id: 11006,
                text: Arc::new("(".into()),
                not: false,
                inverted: false,
                property: None,
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 11007,
                optional: true,
            }),
            Rule::Node(Node {
                debug_id: 11008,
                name: Arc::new("rule".into()),
                property: Some(Arc::new("by".into())),
                index: Cell::new(None),
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 11009,
                optional: true,
            }),
            Rule::Token(Token {
                debug_id: 11010,
                text: Arc::new(")".into()),
                not: false,
                inverted: false,
                property: None,
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 11011,
                optional: true,
            }),
            Rule::Token(Token {
                debug_id: 11012,
                text: Arc::new("{".into()),
                not: false,
                inverted: false,
                property: None,
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 11013,
                optional: true,
            }),
            Rule::Node(Node {
                debug_id: 11014,
                name: Arc::new("rule".into()),
                property: Some(Arc::new("rule".into())),
                index: Cell::new(None),
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 11015,
                optional: true,
            }),
            Rule::Token(Token {
                debug_id: 11016,
                text: Arc::new("}".into()),
                not: false,
                inverted: false,
                property: None,
            }),
        ]
    });

    // 12 "token" [?"!""not" @"set""text" ?[?"!"inv @"set"prop]]
    let token_rule = Rule::Sequence(Sequence {
        debug_id: 12001,
        args: vec![
            Rule::Optional(Box::new(Optional {
                debug_id: 12002,
                rule: Rule::Token(Token {
                    debug_id: 12003,
                    text: Arc::new("!".into()),
                    not: false,
                    inverted: false,
                    property: Some(Arc::new("not".into()))
                })
            })),
            Rule::Node(Node {
                debug_id: 12004,
                name: Arc::new("set".into()),
                property: Some(Arc::new("text".into())),
                index: Cell::new(None),
            }),
            Rule::Optional(Box::new(Optional {
                debug_id: 12005,
                rule: Rule::Sequence(Sequence {
                    debug_id: 12006,
                    args: vec![
                        Rule::Optional(Box::new(Optional {
                            debug_id: 12007,
                            rule: Rule::Token(Token {
                                debug_id: 12008,
                                text: Arc::new("!".into()),
                                not: false,
                                inverted: false,
                                property: Some(inv.clone()),
                            })
                        })),
                        Rule::Node(Node {
                            debug_id: 12009,
                            name: Arc::new("set".into()),
                            property: Some(prop.clone()),
                            index: Cell::new(None),
                        })
                    ]
                })
            })),
        ]
    });

    // 13 "optional" ["?" @"rule""rule"]
    let optional_rule = Rule::Sequence(Sequence {
        debug_id: 13001,
        args: vec![
            Rule::Token(Token {
                debug_id: 13002,
                text: Arc::new("?".into()),
                not: false,
                inverted: false,
                property: None,
            }),
            Rule::Node(Node {
                debug_id: 13003,
                name: Arc::new("rule".into()),
                property: Some(Arc::new("rule".into())),
                index: Cell::new(None),
            })
        ]
    });

    // 14 "whitespace" ["w" @"opt"]
    let whitespace_rule = Rule::Sequence(Sequence {
        debug_id: 14001,
        args: vec![
            Rule::Token(Token {
                debug_id: 14002,
                text: Arc::new("w".into()),
                not: false,
                inverted: false,
                property: None,
            }),
            Rule::Node(Node {
                debug_id: 14003,
                name: Arc::new("opt".into()),
                property: None,
                index: Cell::new(None),
            })
        ]
    });

    // 15 "until_any_or_whitespace" [".." @"set_opt"any @"opt" ?@"set"prop]
    let until_any_or_whitespace_rule = Rule::Sequence(Sequence {
        debug_id: 15001,
        args: vec![
            Rule::Token(Token {
                debug_id: 15002,
                text: Arc::new("..".into()),
                not: false,
                inverted: false,
                property: None,
            }),
            Rule::Node(Node {
                debug_id: 15003,
                name: Arc::new("set_opt".into()),
                property: Some(any.clone()),
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                debug_id: 15004,
                name: Arc::new("opt".into()),
                property: None,
                index: Cell::new(None),
            }),
            Rule::Optional(Box::new(Optional {
                debug_id: 15005,
                rule: Rule::Node(Node {
                    debug_id: 15006,
                    name: Arc::new("set".into()),
                    property: Some(prop.clone()),
                    index: Cell::new(None),
                })
            }))
        ]
    });

    // 16 "until_any" ["..." @"set_opt"any @"opt" ?@"set"prop]
    let until_any_rule = Rule::Sequence(Sequence {
        debug_id: 16001,
        args: vec![
            Rule::Token(Token {
                debug_id: 16002,
                text: Arc::new("...".into()),
                not: false,
                inverted: false,
                property: None,
            }),
            Rule::Node(Node {
                debug_id: 16003,
                name: Arc::new("set_opt".into()),
                property: Some(any.clone()),
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                debug_id: 16004,
                name: Arc::new("opt".into()),
                property: None,
                index: Cell::new(None),
            }),
            Rule::Optional(Box::new(Optional {
                debug_id: 16005,
                rule: Rule::Node(Node {
                    debug_id: 16006,
                    name: Arc::new("set".into()),
                    property: Some(prop.clone()),
                    index: Cell::new(None),
                })
            }))
        ]
    });

    // 17 "repeat" ["r" @"opt" "(" @"rule""rule" ")"]
    let repeat_rule = Rule::Sequence(Sequence {
        debug_id: 17001,
        args: vec![
            Rule::Token(Token {
                debug_id: 17002,
                text: Arc::new("r".into()),
                not: false,
                inverted: false,
                property: None,
            }),
            Rule::Node(Node {
                debug_id: 17003,
                name: Arc::new("opt".into()),
                property: None,
                index: Cell::new(None),
            }),
            Rule::Token(Token {
                debug_id: 17004,
                text: Arc::new("(".into()),
                not: false,
                inverted: false,
                property: None,
            }),
            Rule::Node(Node {
                debug_id: 17005,
                name: Arc::new("rule".into()),
                property: Some(Arc::new("rule".into())),
                index: Cell::new(None),
            }),
            Rule::Token(Token {
                debug_id: 17006,
                text: Arc::new(")".into()),
                not: false,
                inverted: false,
                property: None,
            })
        ]
    });

    // 18 "lines" ["l(" w? @"rule""rule" w? ")"]
    let lines_rule = Rule::Sequence(Sequence {
        debug_id: 18001,
        args: vec![
            Rule::Token(Token {
                debug_id: 18002,
                text: Arc::new("l(".into()),
                not: false,
                inverted: false,
                property: None,
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 18003,
                optional: true,
            }),
            Rule::Node(Node {
                debug_id: 18004,
                name: Arc::new("rule".into()),
                property: Some(Arc::new("rule".into())),
                index: Cell::new(None),
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 18005,
                optional: true,
            }),
            Rule::Token(Token {
                debug_id: 18006,
                text: Arc::new(")".into()),
                not: false,
                inverted: false,
                property: None,
            })
        ]
    });

    /*
    19 "rule" {
      @"whitespace""whitespace"
      @"until_any_or_whitespace""until_any_or_whitespace"
      @"until_any""until_any"
      @"lines""lines"
      @"repeat""repeat"
      @"number""number"
      @"text""text"
      @"reference""reference"
      @"sequence""sequence"
      @"select""select"
      @"separated_by""separated_by"
      @"token""token"
      @"optional""optional"
    }
    */
    let rule_rule = Rule::Select(Select {
        debug_id: 19001,
        args: vec![
            Rule::Node(Node {
                debug_id: 19001,
                name: Arc::new("whitespace".into()),
                property: Some(Arc::new("whitespace".into())),
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                debug_id: 19002,
                name: Arc::new("until_any_or_whitespace".into()),
                property: Some(Arc::new("until_any_or_whitespace".into())),
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                debug_id: 19003,
                name: Arc::new("until_any".into()),
                property: Some(Arc::new("until_any".into())),
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                debug_id: 19004,
                name: Arc::new("lines".into()),
                property: Some(Arc::new("lines".into())),
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                debug_id: 19005,
                name: Arc::new("repeat".into()),
                property: Some(Arc::new("repeat".into())),
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                debug_id: 19006,
                name: Arc::new("number".into()),
                property: Some(Arc::new("number".into())),
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                debug_id: 19007,
                name: Arc::new("text".into()),
                property: Some(Arc::new("text".into())),
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                debug_id: 19008,
                name: Arc::new("reference".into()),
                property: Some(Arc::new("reference".into())),
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                debug_id: 19009,
                name: Arc::new("sequence".into()),
                property: Some(Arc::new("sequence".into())),
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                debug_id: 19010,
                name: Arc::new("select".into()),
                property: Some(Arc::new("select".into())),
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                debug_id: 19011,
                name: Arc::new("separated_by".into()),
                property: Some(Arc::new("separated_by".into())),
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                debug_id: 19012,
                name: Arc::new("token".into()),
                property: Some(Arc::new("token".into())),
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                debug_id: 19013,
                name: Arc::new("optional".into()),
                property: Some(Arc::new("optional".into())),
                index: Cell::new(None),
            }),
        ]
    });

    // 20 "document" [l(@"string""string") l(@"node""node") w?]
    let document_rule = Rule::Sequence(Sequence {
        debug_id: 20001,
        args: vec![
            Rule::Lines(Box::new(Lines {
                debug_id: 20002,
                rule: Rule::Node(Node {
                    debug_id: 20003,
                    name: Arc::new("string".into()),
                    property: Some(Arc::new("string".into())),
                    index: Cell::new(None),
                })
            })),
            Rule::Lines(Box::new(Lines {
                debug_id: 20004,
                rule: Rule::Node(Node {
                    debug_id: 20005,
                    name: Arc::new("node".into()),
                    property: Some(Arc::new("node".into())),
                    index: Cell::new(None),
                })
            })),
            Rule::Whitespace(Whitespace {
                debug_id: 20006,
                optional: true,
            })
        ]
    });

    let mut syntax = Syntax {
        rules: Vec::with_capacity(20),
        names: Vec::with_capacity(20)
    };
    syntax.push((Arc::new("string".into()), string_rule));
    syntax.push((Arc::new("node".into()), node_rule));
    syntax.push((Arc::new("set".into()), set_rule));
    syntax.push((Arc::new("set_opt".into()), set_opt_rule));
    syntax.push((Arc::new("opt".into()), opt_rule));
    syntax.push((Arc::new("number".into()), number_rule));
    syntax.push((Arc::new("text".into()), text_rule));
    syntax.push((Arc::new("reference".into()), reference_rule));
    syntax.push((Arc::new("sequence".into()), sequence_rule));
    syntax.push((Arc::new("select".into()), select_rule));
    syntax.push((Arc::new("separated_by".into()), separated_by_rule));
    syntax.push((Arc::new("token".into()), token_rule));
    syntax.push((Arc::new("optional".into()), optional_rule));
    syntax.push((Arc::new("whitespace".into()), whitespace_rule));
    syntax.push((Arc::new("until_any_or_whitespace".into()), until_any_or_whitespace_rule));
    syntax.push((Arc::new("until_any".into()), until_any_rule));
    syntax.push((Arc::new("repeat".into()), repeat_rule));
    syntax.push((Arc::new("lines".into()), lines_rule));
    syntax.push((Arc::new("rule".into()), rule_rule));
    syntax.push((Arc::new("document".into()), document_rule));
    update_refs(&mut syntax);
    syntax
}
