use std::{collections::HashMap, fmt::Display};

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum Vocabulary {
    Word = 0,
    Semicolon,
    Newline,
    And,
    Or,
    OpenBrace,
    CloseBrace,
    OpenParenthese,
    DollarOpenParenthese,
    DollarOpenParentheseParenthese,
    CloseParenthese,
    CloseParentheseParenthese,
    Backquote,
    For,
    In,
    Do,
    Done,
    While,
    Until,
    Case,
    Esac,
    If,
    Then,
    Elif,
    Else,
    Fi,
    DoubleSemicolon,
    IoNumber, // FD to redirect
    InfInf,
    InfInfMin,
    RSup, // >
    RSupPipe, // >|
    RSupSup, // >>
    RSupAnd, // >&
    RInf, // <
    RInfAnd, // <&
    RInfSup, // <>
    RPipe, // |
    Neg, // !
    Varassignment, // test=3
    Heredoc,
}

impl Display for Vocabulary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}


pub fn generate_mapper() -> HashMap<Vocabulary, String> {
    HashMap::from([
        (Vocabulary::Word, String::from("")),
        (Vocabulary::Semicolon, String::from(";")),
        (Vocabulary::Newline, String::from("\n")),
        (Vocabulary::And, String::from("&&")),
        (Vocabulary::Or, String::from("||")),
        (Vocabulary::OpenBrace, String::from("{")),
        (Vocabulary::CloseBrace, String::from("}")),
        (Vocabulary::OpenParenthese, String::from("(")),
        (Vocabulary::DollarOpenParenthese, String::from("$(")),
        (Vocabulary::DollarOpenParentheseParenthese, String::from("$((")),
        (Vocabulary::CloseParenthese, String::from(")")),
        (Vocabulary::CloseParentheseParenthese, String::from("))")),
        (Vocabulary::Backquote, String::from("`")),
        (Vocabulary::For, String::from("for")),
        (Vocabulary::In, String::from("in")),
        (Vocabulary::Do, String::from("do")),
        (Vocabulary::Done, String::from("done")),
        (Vocabulary::While, String::from("while")),
        (Vocabulary::Until, String::from("until")),
        (Vocabulary::Case, String::from("case")),
        (Vocabulary::Esac, String::from("esac")),
        (Vocabulary::If, String::from("if")),
        (Vocabulary::Then, String::from("then")),
        (Vocabulary::Elif, String::from("elif")),
        (Vocabulary::Else, String::from("else")),
        (Vocabulary::Fi, String::from("fi")),
        (Vocabulary::DoubleSemicolon, String::from(";;")),
        (Vocabulary::IoNumber, String::from("")),
        (Vocabulary::InfInf, String::from("<<")),
        (Vocabulary::InfInfMin, String::from("<<-")),
        (Vocabulary::RSup, String::from(">")),
        (Vocabulary::RSupPipe, String::from(">|")),
        (Vocabulary::RSupSup, String::from(">>")),
        (Vocabulary::RSupAnd, String::from(">&")),
        (Vocabulary::RInf, String::from("<")),
        (Vocabulary::RInfAnd, String::from("<&")),
        (Vocabulary::RInfSup, String::from("<>")),
        (Vocabulary::RPipe, String::from("|")),
        (Vocabulary::Neg, String::from("!")),
        (Vocabulary::Varassignment, String::from("")),
        (Vocabulary::Heredoc, String::from("")),
    ])
}
