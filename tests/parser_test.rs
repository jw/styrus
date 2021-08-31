extern crate pest;
#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "styrus.pest"]
struct StylusParser;

use pest::{consumes_to, parses_to};

#[test]
fn nothing() {
    parses_to! {
        parser: StylusParser,
        input: "",
        rule: Rule::stylus,
        tokens: [
            stylus(0, 0, [
                rules(0, 0, []),
                EOI(0, 0)
            ])
        ]
    }
}

#[test]
fn one_empty_line() {
    parses_to! {
        parser: StylusParser,
        input: "\n",
        rule: Rule::stylus,
        tokens: [
            stylus(0, 1, [
                rules(0, 1, [
                    rule(0, 1, [])
                ]),
                EOI(1, 1)
            ])
        ]
    }
}

#[test]
fn complete() {
    parses_to! {
        parser: StylusParser,
        input: "*h1 > p\n  border 1px\n\nh2\n  padding 1px 1px 1px 1px\n",
        rule: Rule::stylus,
        tokens: [
            stylus(0, 51, [
                rules(0, 51, [
                    rule(0, 21, [
                        selectors(0, 7, [
                            selector(0, 1, [
                                asterisk(0, 1)
                            ]),
                            selector(1, 5, [
                                identifier(1, 3, []),
                                separator(4, 5, [])
                            ]),
                            selector(6, 7, [
                                identifier(6, 7, [])
                            ])
                        ]),
                        properties(7, 20, [
                            property(7, 20, [
                                indent(7, 10, []),
                                name(10, 16, []),
                                value(17, 20, [])
                            ])
                        ])
                    ]),
                    rule(21, 22, []),
                    rule(22, 51, [
                        selectors(22, 24, [
                            selector(22, 24, [
                                identifier(22, 24)
                            ])
                        ]),
                        properties(24, 50, [
                            property(24, 50, [
                                indent(24, 27),
                                name(27, 34),
                                value(35, 38),
                                value(39, 42),
                                value(43, 46),
                                value(47, 50),

                            ])
                        ])
                    ])
                ]),
                EOI(51, 51),
            ])
        ]
    }
}
