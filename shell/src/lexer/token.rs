use super::vocab::Vocabulary;
use std::fmt;


#[derive(Debug, Clone)]
#[derive(PartialEq)]
pub struct Token {
    pub representation: String,
    pub vocab_type: Vocabulary,
}

impl Token {
    pub fn new(representation: String, vocab_type: Vocabulary) -> Self {
        Token {
            representation,
            vocab_type,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Token: {} -> {}", self.representation, self.vocab_type)
    }
}
