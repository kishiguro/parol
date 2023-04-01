// ---------------------------------------------------------
// This file was generated by parol.
// It is not intended for manual editing and changes will be
// lost after next build.
// ---------------------------------------------------------

use parol_runtime::once_cell::sync::Lazy;
#[allow(unused_imports)]
use parol_runtime::parser::{
    LLKParser, LookaheadDFA, ParseTreeType, ParseType, Production, Trans, UserActionsTrait,
};
use parol_runtime::{ParolError, ParseTree};
use parol_runtime::{TokenStream, Tokenizer};
use std::cell::RefCell;
use std::path::Path;

use parol_runtime::lexer::tokenizer::{
    ERROR_TOKEN, NEW_LINE_TOKEN, UNMATCHABLE_TOKEN, WHITESPACE_TOKEN,
};

pub const TERMINALS: &[&str; 11] = &[
    /*  0 */ UNMATCHABLE_TOKEN,
    /*  1 */ UNMATCHABLE_TOKEN,
    /*  2 */ UNMATCHABLE_TOKEN,
    /*  3 */ UNMATCHABLE_TOKEN,
    /*  4 */ UNMATCHABLE_TOKEN,
    /*  5 */ r###";"###,
    /*  6 */ r###"[a-zA-Z_][a-zA-Z0-9_]*"###,
    /*  7 */ r###"(?i)(?-u:\b)Begin(?-u:\b)"###,
    /*  8 */ r###"(?i)(?-u:\b)End(?-u:\b)"###,
    /*  9 */ r###"(?i)(?-u:\b)Var(?-u:\b)"###,
    /* 10 */ ERROR_TOKEN,
];

pub const TERMINAL_NAMES: &[&str; 11] = &[
    /*  0 */ "EndOfInput",
    /*  1 */ "Newline",
    /*  2 */ "Whitespace",
    /*  3 */ "LineComment",
    /*  4 */ "BlockComment",
    /*  5 */ "Semicolon",
    /*  6 */ "Identifier",
    /*  7 */ "Begin",
    /*  8 */ "End",
    /*  9 */ "Var",
    /* 10 */ "Error",
];

/* SCANNER_0: "INITIAL" */
const SCANNER_0: (&[&str; 5], &[usize; 4]) = (
    &[
        /*  0 */ UNMATCHABLE_TOKEN,
        /*  1 */ NEW_LINE_TOKEN,
        /*  2 */ WHITESPACE_TOKEN,
        /*  3 */ UNMATCHABLE_TOKEN,
        /*  4 */ UNMATCHABLE_TOKEN,
    ],
    &[
        5, /* Semicolon */
        7, /* Begin */
        8, /* End */
        9, /* Var */
    ],
);

/* SCANNER_1: "Identifier" */
const SCANNER_1: (&[&str; 5], &[usize; 1]) = (
    &[
        /*  0 */ UNMATCHABLE_TOKEN,
        /*  1 */ NEW_LINE_TOKEN,
        /*  2 */ WHITESPACE_TOKEN,
        /*  3 */ UNMATCHABLE_TOKEN,
        /*  4 */ UNMATCHABLE_TOKEN,
    ],
    &[6 /* Identifier */],
);

const MAX_K: usize = 1;

pub const NON_TERMINALS: &[&str; 10] = &[
    /*  0 */ "Begin",
    /*  1 */ "Block",
    /*  2 */ "BlockList",
    /*  3 */ "Declaration",
    /*  4 */ "End",
    /*  5 */ "Grammar",
    /*  6 */ "GrammarList",
    /*  7 */ "Identifier",
    /*  8 */ "Items",
    /*  9 */ "Var",
];

pub const LOOKAHEAD_AUTOMATA: &[LookaheadDFA; 10] = &[
    /* 0 - "Begin" */
    LookaheadDFA {
        prod0: 10,
        transitions: &[],
        k: 0,
    },
    /* 1 - "Block" */
    LookaheadDFA {
        prod0: 6,
        transitions: &[],
        k: 0,
    },
    /* 2 - "BlockList" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[Trans(0, 7, 1, 7), Trans(0, 8, 2, 8), Trans(0, 9, 1, 7)],
        k: 1,
    },
    /* 3 - "Declaration" */
    LookaheadDFA {
        prod0: 5,
        transitions: &[],
        k: 0,
    },
    /* 4 - "End" */
    LookaheadDFA {
        prod0: 11,
        transitions: &[],
        k: 0,
    },
    /* 5 - "Grammar" */
    LookaheadDFA {
        prod0: 0,
        transitions: &[],
        k: 0,
    },
    /* 6 - "GrammarList" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[Trans(0, 0, 2, 2), Trans(0, 7, 1, 1), Trans(0, 9, 1, 1)],
        k: 1,
    },
    /* 7 - "Identifier" */
    LookaheadDFA {
        prod0: 9,
        transitions: &[],
        k: 0,
    },
    /* 8 - "Items" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[Trans(0, 7, 2, 4), Trans(0, 9, 1, 3)],
        k: 1,
    },
    /* 9 - "Var" */
    LookaheadDFA {
        prod0: 12,
        transitions: &[],
        k: 0,
    },
];

pub const PRODUCTIONS: &[Production; 13] = &[
    // 0 - Grammar: GrammarList /* Vec */;
    Production {
        lhs: 5,
        production: &[ParseType::N(6)],
    },
    // 1 - GrammarList: Items GrammarList;
    Production {
        lhs: 6,
        production: &[ParseType::N(6), ParseType::N(8)],
    },
    // 2 - GrammarList: ;
    Production {
        lhs: 6,
        production: &[],
    },
    // 3 - Items: Declaration;
    Production {
        lhs: 8,
        production: &[ParseType::N(3)],
    },
    // 4 - Items: Block;
    Production {
        lhs: 8,
        production: &[ParseType::N(1)],
    },
    // 5 - Declaration: Var S(1) Identifier S(0) ";";
    Production {
        lhs: 3,
        production: &[
            ParseType::T(5),
            ParseType::S(0),
            ParseType::N(7),
            ParseType::S(1),
            ParseType::N(9),
        ],
    },
    // 6 - Block: Begin BlockList /* Vec */ End;
    Production {
        lhs: 1,
        production: &[ParseType::N(4), ParseType::N(2), ParseType::N(0)],
    },
    // 7 - BlockList: Items BlockList;
    Production {
        lhs: 2,
        production: &[ParseType::N(2), ParseType::N(8)],
    },
    // 8 - BlockList: ;
    Production {
        lhs: 2,
        production: &[],
    },
    // 9 - Identifier: "[a-zA-Z_][a-zA-Z0-9_]*";
    Production {
        lhs: 7,
        production: &[ParseType::T(6)],
    },
    // 10 - Begin: "(?i)(?-u:\b)Begin(?-u:\b)";
    Production {
        lhs: 0,
        production: &[ParseType::T(7)],
    },
    // 11 - End: "(?i)(?-u:\b)End(?-u:\b)";
    Production {
        lhs: 4,
        production: &[ParseType::T(8)],
    },
    // 12 - Var: "(?i)(?-u:\b)Var(?-u:\b)";
    Production {
        lhs: 9,
        production: &[ParseType::T(9)],
    },
];

static TOKENIZERS: Lazy<Vec<(&'static str, Tokenizer)>> = Lazy::new(|| {
    vec![
        (
            "INITIAL",
            Tokenizer::build(TERMINALS, SCANNER_0.0, SCANNER_0.1).unwrap(),
        ),
        (
            "Identifier",
            Tokenizer::build(TERMINALS, SCANNER_1.0, SCANNER_1.1).unwrap(),
        ),
    ]
});

pub fn parse<'t, T>(
    input: &'t str,
    file_name: T,
    user_actions: &mut dyn UserActionsTrait<'t>,
) -> Result<ParseTree<'t>, ParolError>
where
    T: AsRef<Path>,
{
    let mut llk_parser = LLKParser::new(
        5,
        LOOKAHEAD_AUTOMATA,
        PRODUCTIONS,
        TERMINAL_NAMES,
        NON_TERMINALS,
    );
    let token_stream =
        RefCell::new(TokenStream::new(input, file_name, &TOKENIZERS, MAX_K).unwrap());
    llk_parser.parse(token_stream, user_actions)
}
