use std::rc::Rc;
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

/// Returns rules for parsing meta rules.
pub fn rules() -> Vec<(Rc<String>, Rule)> {
    let opt: Rc<String> = Rc::new("optional".into());
    let inv: Rc<String> = Rc::new("inverted".into());
    let prop: Rc<String> = Rc::new("property".into());
    let any: Rc<String> = Rc::new("any_characters".into());
    let seps: Rc<String> = Rc::new("[]{}():.!?\"".into());

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
                property: Some(Rc::new("name".into()))
            }),
            Rule::Token(Token {
                debug_id: 1004,
                text: Rc::new(":".into()),
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
                property: Some(Rc::new("text".into())),
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
                property: Some(Rc::new("id".into())),
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 2004,
                optional: false,
            }),
            Rule::Text(Text {
                debug_id: 2005,
                allow_empty: false,
                property: Some(Rc::new("name".into())),
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 2006,
                optional: false,
            }),
            Rule::Node(Node {
                debug_id: 2007,
                name: Rc::new("rule".into()),
                property: Some(Rc::new("rule".into())),
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
                property: Some(Rc::new("value".into())),
            }),
            Rule::UntilAnyOrWhitespace(UntilAnyOrWhitespace {
                debug_id: 3003,
                any_characters: seps.clone(),
                optional: false,
                property: Some(Rc::new("ref".into())),
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
                property: Some(Rc::new("value".into())),
            }),
            Rule::UntilAnyOrWhitespace(UntilAnyOrWhitespace {
                debug_id: 4003,
                any_characters: seps.clone(),
                optional: false,
                property: Some(Rc::new("ref".into())),
            })
        ]
    });

    // 5 "opt" {"?"opt "!"!opt}
    let opt_rule = Rule::Select(Select {
        debug_id: 5001,
        args: vec![
            Rule::Token(Token {
                debug_id: 5002,
                text: Rc::new("?".into()),
                not: false,
                inverted: false,
                property: Some(opt.clone())
            }),
            Rule::Token(Token {
                debug_id: 5003,
                text: Rc::new("!".into()),
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
                text: Rc::new("$".into()),
                not: false,
                inverted: false,
                property: None,
            }),
            Rule::Optional(Box::new(Optional {
                debug_id: 6003,
                rule: Rule::Token(Token {
                    debug_id: 6004,
                    text: Rc::new("_".into()),
                    not: false,
                    inverted: false,
                    property: Some(Rc::new("underscore".into()))
                })
            })),
            Rule::Optional(Box::new(Optional {
                debug_id: 6005,
                rule: Rule::Node(Node {
                    debug_id: 6006,
                    name: Rc::new("set".into()),
                    property: Some(Rc::new("property".into())),
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
                text: Rc::new("t".into()),
                not: false,
                inverted: false,
                property: None,
            }),
            Rule::Select(Select {
                debug_id: 7003,
                args: vec![
                    Rule::Token(Token {
                        debug_id: 7004,
                        text: Rc::new("?".into()),
                        not: false,
                        inverted: false,
                        property: Some(Rc::new("allow_empty".into())),
                    }),
                    Rule::Token(Token {
                        debug_id: 7005,
                        text: Rc::new("!".into()),
                        not: false,
                        inverted: true,
                        property: Some(Rc::new("allow_empty".into())),
                    })
                ]
            }),
            Rule::Optional(Box::new(Optional {
                debug_id: 7006,
                rule: Rule::Node(Node {
                    debug_id: 7007,
                    name: Rc::new("set".into()),
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
                text: Rc::new("@".into()),
                not: false,
                inverted: false,
                property: None,
            }),
            Rule::Text(Text {
                debug_id: 8003,
                allow_empty: false,
                property: Some(Rc::new("name".into())),
            }),
            Rule::Optional(Box::new(Optional {
                debug_id: 8004,
                rule: Rule::Node(Node {
                    debug_id: 8005,
                    name: Rc::new("set".into()),
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
                text: Rc::new("[".into()),
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
                    name: Rc::new("rule".into()),
                    property: Some(Rc::new("rule".into())),
                    index: Cell::new(None)
                })
            })),
            Rule::Token(Token {
                debug_id: 9007,
                text: Rc::new("]".into()),
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
                text: Rc::new("{".into()),
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
                    name: Rc::new("rule".into()),
                    property: Some(Rc::new("rule".into())),
                    index: Cell::new(None),
                })
            })),
            Rule::Token(Token {
                debug_id: 10007,
                text: Rc::new("}".into()),
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
                text: Rc::new("s".into()),
                not: false,
                inverted: false,
                property: None,
            }),
            Rule::Node(Node {
                debug_id: 11003,
                name: Rc::new("opt".into()),
                property: None,
                index: Cell::new(None),
            }),
            Rule::Optional(Box::new(Optional {
                debug_id: 11004,
                rule: Rule::Token(Token {
                    debug_id: 11005,
                    text: Rc::new(".".into()),
                    not: false,
                    inverted: false,
                    property: Some(Rc::new("allow_trail".into())),
                })
            })),
            Rule::Token(Token {
                debug_id: 11006,
                text: Rc::new("(".into()),
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
                name: Rc::new("rule".into()),
                property: Some(Rc::new("by".into())),
                index: Cell::new(None),
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 11009,
                optional: true,
            }),
            Rule::Token(Token {
                debug_id: 11010,
                text: Rc::new(")".into()),
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
                text: Rc::new("{".into()),
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
                name: Rc::new("rule".into()),
                property: Some(Rc::new("rule".into())),
                index: Cell::new(None),
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 11015,
                optional: true,
            }),
            Rule::Token(Token {
                debug_id: 11016,
                text: Rc::new("}".into()),
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
                    text: Rc::new("!".into()),
                    not: false,
                    inverted: false,
                    property: Some(Rc::new("not".into()))
                })
            })),
            Rule::Node(Node {
                debug_id: 12004,
                name: Rc::new("set".into()),
                property: Some(Rc::new("text".into())),
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
                                text: Rc::new("!".into()),
                                not: false,
                                inverted: false,
                                property: Some(inv.clone()),
                            })
                        })),
                        Rule::Node(Node {
                            debug_id: 12009,
                            name: Rc::new("set".into()),
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
                text: Rc::new("?".into()),
                not: false,
                inverted: false,
                property: None,
            }),
            Rule::Node(Node {
                debug_id: 13003,
                name: Rc::new("rule".into()),
                property: Some(Rc::new("rule".into())),
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
                text: Rc::new("w".into()),
                not: false,
                inverted: false,
                property: None,
            }),
            Rule::Node(Node {
                debug_id: 14003,
                name: Rc::new("opt".into()),
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
                text: Rc::new("..".into()),
                not: false,
                inverted: false,
                property: None,
            }),
            Rule::Node(Node {
                debug_id: 15003,
                name: Rc::new("set_opt".into()),
                property: Some(any.clone()),
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                debug_id: 15004,
                name: Rc::new("opt".into()),
                property: None,
                index: Cell::new(None),
            }),
            Rule::Optional(Box::new(Optional {
                debug_id: 15005,
                rule: Rule::Node(Node {
                    debug_id: 15006,
                    name: Rc::new("set".into()),
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
                text: Rc::new("...".into()),
                not: false,
                inverted: false,
                property: None,
            }),
            Rule::Node(Node {
                debug_id: 16003,
                name: Rc::new("set_opt".into()),
                property: Some(any.clone()),
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                debug_id: 16004,
                name: Rc::new("opt".into()),
                property: None,
                index: Cell::new(None),
            }),
            Rule::Optional(Box::new(Optional {
                debug_id: 16005,
                rule: Rule::Node(Node {
                    debug_id: 16006,
                    name: Rc::new("set".into()),
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
                text: Rc::new("r".into()),
                not: false,
                inverted: false,
                property: None,
            }),
            Rule::Node(Node {
                debug_id: 17003,
                name: Rc::new("opt".into()),
                property: None,
                index: Cell::new(None),
            }),
            Rule::Token(Token {
                debug_id: 17004,
                text: Rc::new("(".into()),
                not: false,
                inverted: false,
                property: None,
            }),
            Rule::Node(Node {
                debug_id: 17005,
                name: Rc::new("rule".into()),
                property: Some(Rc::new("rule".into())),
                index: Cell::new(None),
            }),
            Rule::Token(Token {
                debug_id: 17006,
                text: Rc::new(")".into()),
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
                text: Rc::new("l(".into()),
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
                name: Rc::new("rule".into()),
                property: Some(Rc::new("rule".into())),
                index: Cell::new(None),
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 18005,
                optional: true,
            }),
            Rule::Token(Token {
                debug_id: 18006,
                text: Rc::new(")".into()),
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
                name: Rc::new("whitespace".into()),
                property: Some(Rc::new("whitespace".into())),
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                debug_id: 19002,
                name: Rc::new("until_any_or_whitespace".into()),
                property: Some(Rc::new("until_any_or_whitespace".into())),
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                debug_id: 19003,
                name: Rc::new("until_any".into()),
                property: Some(Rc::new("until_any".into())),
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                debug_id: 19004,
                name: Rc::new("lines".into()),
                property: Some(Rc::new("lines".into())),
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                debug_id: 19005,
                name: Rc::new("repeat".into()),
                property: Some(Rc::new("repeat".into())),
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                debug_id: 19006,
                name: Rc::new("number".into()),
                property: Some(Rc::new("number".into())),
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                debug_id: 19007,
                name: Rc::new("text".into()),
                property: Some(Rc::new("text".into())),
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                debug_id: 19008,
                name: Rc::new("reference".into()),
                property: Some(Rc::new("reference".into())),
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                debug_id: 19009,
                name: Rc::new("sequence".into()),
                property: Some(Rc::new("sequence".into())),
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                debug_id: 19010,
                name: Rc::new("select".into()),
                property: Some(Rc::new("select".into())),
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                debug_id: 19011,
                name: Rc::new("separated_by".into()),
                property: Some(Rc::new("separated_by".into())),
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                debug_id: 19012,
                name: Rc::new("token".into()),
                property: Some(Rc::new("token".into())),
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                debug_id: 19013,
                name: Rc::new("optional".into()),
                property: Some(Rc::new("optional".into())),
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
                    name: Rc::new("string".into()),
                    property: Some(Rc::new("string".into())),
                    index: Cell::new(None),
                })
            })),
            Rule::Lines(Box::new(Lines {
                debug_id: 20004,
                rule: Rule::Node(Node {
                    debug_id: 20005,
                    name: Rc::new("node".into()),
                    property: Some(Rc::new("node".into())),
                    index: Cell::new(None),
                })
            })),
            Rule::Whitespace(Whitespace {
                debug_id: 20006,
                optional: true,
            })
        ]
    });

    let rules = vec![
        (Rc::new("string".into()), string_rule),
        (Rc::new("node".into()), node_rule),
        (Rc::new("set".into()), set_rule),
        (Rc::new("set_opt".into()), set_opt_rule),
        (Rc::new("opt".into()), opt_rule),
        (Rc::new("number".into()), number_rule),
        (Rc::new("text".into()), text_rule),
        (Rc::new("reference".into()), reference_rule),
        (Rc::new("sequence".into()), sequence_rule),
        (Rc::new("select".into()), select_rule),
        (Rc::new("separated_by".into()), separated_by_rule),
        (Rc::new("token".into()), token_rule),
        (Rc::new("optional".into()), optional_rule),
        (Rc::new("whitespace".into()), whitespace_rule),
        (Rc::new("until_any_or_whitespace".into()), until_any_or_whitespace_rule),
        (Rc::new("until_any".into()), until_any_rule),
        (Rc::new("repeat".into()), repeat_rule),
        (Rc::new("lines".into()), lines_rule),
        (Rc::new("rule".into()), rule_rule),
        (Rc::new("document".into()), document_rule),
    ];
    update_refs(&rules);
    rules
}
