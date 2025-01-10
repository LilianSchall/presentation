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
    Amperstand, // &
    Neg, // !
    Varassignment, // test=3
    Heredoc,
}

impl Display for Vocabulary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}


pub fn generate_mapper() -> HashMap<String, Vocabulary> {
    HashMap::from([
        (String::from(";"),Vocabulary::Semicolon),
        (String::from("\n"),Vocabulary::Newline),
        (String::from("&&"),Vocabulary::And),
        (String::from("||"),Vocabulary::Or),
        (String::from("{"),Vocabulary::OpenBrace),
        (String::from("}"),Vocabulary::CloseBrace),
        (String::from("("),Vocabulary::OpenParenthese),
        (String::from(")"),Vocabulary::CloseParenthese),
        (String::from("`"),Vocabulary::Backquote),
        (String::from("for"),Vocabulary::For),
        (String::from("in"),Vocabulary::In),
        (String::from("do"),Vocabulary::Do),
        (String::from("done"),Vocabulary::Done),
        (String::from("while"),Vocabulary::While),
        (String::from("until"),Vocabulary::Until),
        (String::from("case"),Vocabulary::Case),
        (String::from("esac"),Vocabulary::Esac),
        (String::from("if"),Vocabulary::If),
        (String::from("then"),Vocabulary::Then),
        (String::from("elif"),Vocabulary::Elif),
        (String::from("else"),Vocabulary::Else),
        (String::from("fi"),Vocabulary::Fi),
        (String::from(";;"),Vocabulary::DoubleSemicolon),
        (String::from("<<"),Vocabulary::InfInf),
        (String::from("<<-"),Vocabulary::InfInfMin),
        (String::from(">"),Vocabulary::RSup),
        (String::from(">|"),Vocabulary::RSupPipe),
        (String::from(">>"),Vocabulary::RSupSup),
        (String::from(">&"),Vocabulary::RSupAnd),
        (String::from("<"),Vocabulary::RInf),
        (String::from("<&"),Vocabulary::RInfAnd),
        (String::from("<>"),Vocabulary::RInfSup),
        (String::from("|"),Vocabulary::RPipe),
        (String::from("&"),Vocabulary::Amperstand),
        (String::from("!"),Vocabulary::Neg),
    ])
}

pub fn generate_separators() -> Vec<char> {
    vec![' ', '\t', '\n', '\r', ';', '(', ')', '{', '}', '`', '|', '&', '<', '>', '!']
}
