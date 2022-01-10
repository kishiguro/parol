// ---------------------------------------------------------
// This file was generated by parol.
// It is not intended for manual editing and changes will be
// lost after next build.
// ---------------------------------------------------------

use id_tree::Tree;
use miette::Result;
use parol_runtime::lexer::{TokenStream, Tokenizer};
use parol_runtime::parser::{
    DFATransition, LLKParser, LookaheadDFA, ParseTreeType, ParseType, Production, UserActionsTrait,
};
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
    /*  6 */ r###"(?i)\bBegin\b"###,
    /*  7 */ r###"(?i)\bEnd\b"###,
    /*  8 */ r###"(?i)\bVar\b"###,
    /*  9 */ r###"[a-zA-Z_][a-zA-Z0-9_]*"###,
    /* 10 */ ERROR_TOKEN,
];

pub const TERMINAL_NAMES: &[&str; 11] = &[
    /*  0 */ "EndOfInput",
    /*  1 */ "Newline",
    /*  2 */ "Whitespace",
    /*  3 */ "LineComment",
    /*  4 */ "BlockComment",
    /*  5 */ "Semicolon",
    /*  6 */ "Begin",
    /*  7 */ "End",
    /*  8 */ "Var",
    /*  9 */ "Identifier",
    /* 10 */ "Error",
];

/* SCANNER_0: "INITIAL" */
const SCANNER_0: (&[&str; 5], &[usize; 5]) = (
    &[
        /*  0 */ UNMATCHABLE_TOKEN,
        /*  1 */ NEW_LINE_TOKEN,
        /*  2 */ WHITESPACE_TOKEN,
        /*  3 */ UNMATCHABLE_TOKEN,
        /*  4 */ UNMATCHABLE_TOKEN,
    ],
    &[
        5, /* Semicolon */
        6, /* Begin */
        7, /* End */
        8, /* Var */
        9, /* Identifier */
    ],
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
        states: &[Some(9)],
        transitions: &[],
        k: 0,
    },
    /* 1 - "Block" */
    LookaheadDFA {
        states: &[Some(6)],
        transitions: &[],
        k: 0,
    },
    /* 2 - "BlockList" */
    LookaheadDFA {
        states: &[None, Some(7), Some(8)],
        transitions: &[
            DFATransition(0, 6, 1),
            DFATransition(0, 7, 2),
            DFATransition(0, 8, 1),
        ],
        k: 1,
    },
    /* 3 - "Declaration" */
    LookaheadDFA {
        states: &[Some(5)],
        transitions: &[],
        k: 0,
    },
    /* 4 - "End" */
    LookaheadDFA {
        states: &[Some(10)],
        transitions: &[],
        k: 0,
    },
    /* 5 - "Grammar" */
    LookaheadDFA {
        states: &[Some(0)],
        transitions: &[],
        k: 0,
    },
    /* 6 - "GrammarList" */
    LookaheadDFA {
        states: &[None, Some(1), Some(2)],
        transitions: &[
            DFATransition(0, 0, 2),
            DFATransition(0, 6, 1),
            DFATransition(0, 8, 1),
        ],
        k: 1,
    },
    /* 7 - "Identifier" */
    LookaheadDFA {
        states: &[Some(12)],
        transitions: &[],
        k: 0,
    },
    /* 8 - "Items" */
    LookaheadDFA {
        states: &[None, Some(3), Some(4)],
        transitions: &[DFATransition(0, 6, 2), DFATransition(0, 8, 1)],
        k: 1,
    },
    /* 9 - "Var" */
    LookaheadDFA {
        states: &[Some(11)],
        transitions: &[],
        k: 0,
    },
];

pub const PRODUCTIONS: &[Production; 13] = &[
    // 0 - Grammar: GrammarList;
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
    // 5 - Declaration: Var Identifier ";";
    Production {
        lhs: 3,
        production: &[ParseType::T(5), ParseType::N(7), ParseType::N(9)],
    },
    // 6 - Block: Begin BlockList End;
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
    // 9 - Begin: "(?i)\bBegin\b";
    Production {
        lhs: 0,
        production: &[ParseType::T(6)],
    },
    // 10 - End: "(?i)\bEnd\b";
    Production {
        lhs: 4,
        production: &[ParseType::T(7)],
    },
    // 11 - Var: "(?i)\bVar\b";
    Production {
        lhs: 9,
        production: &[ParseType::T(8)],
    },
    // 12 - Identifier: "[a-zA-Z_][a-zA-Z0-9_]*";
    Production {
        lhs: 7,
        production: &[ParseType::T(9)],
    },
];

lazy_static! {
    static ref TOKENIZERS: Vec<(&'static str, Tokenizer)> = vec![(
        "INITIAL",
        Tokenizer::build(TERMINALS, SCANNER_0.0, SCANNER_0.1).unwrap()
    ),];
}

pub fn parse<'t, T>(
    input: &'t str,
    file_name: T,
    user_actions: &mut dyn UserActionsTrait,
) -> Result<Tree<ParseTreeType<'t>>>
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
    let result = llk_parser.parse(token_stream, user_actions);
    match result {
        Ok(()) => Ok(llk_parser.parse_tree),
        Err(e) => Err(e),
    }
}
