use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use super::vocab::{Vocabulary, generate_mapper};
use super::token::Token;

pub struct Lexer {
    pub mapper: HashMap<Vocabulary, String>,
    pub position: usize,
    pub input: Vec<char>,
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position >= self.input.len() {
            return None;
        }

        let current: Rc<RefCell<String>> = Rc::new(RefCell::new(String::new()));

        while self.position < self.input.len() && !self.is_whitespace(self.input[self.position]) {
            let c = self.input[self.position];
            self.position += 1;

            match c {
                 '\\' => self.get_escaped_char(Rc::clone(&current)),
                 '\'' => self.get_quoted_string(Rc::clone(&current)),
                 '"' => self.get_double_quoted_string(Rc::clone(&current)),
                 _ => current.borrow_mut().push(c),
            }
        }

        while self.position < self.input.len() && self.is_whitespace(self.input[self.position]) {
            self.position += 1;
        }

        let result: String = current.borrow().clone(); 

        Some(Token::new(result, Vocabulary::Word))
    }
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mapper = generate_mapper();
        Lexer {
            mapper,
            position: 0,
            input: input.chars().collect(),
        }
    }

    fn is_whitespace(&self, c: char) -> bool {
        c == ' ' || c == '\t' || c == '\n' || c == '\r'
    }

    fn get_escaped_char(&mut self, current: Rc<RefCell<String>>) {
        if self.position >= self.input.len() {
            return;
        }

        let c = self.input[self.position];
        match c {
            'n' => current.borrow_mut().push('\n'),
            't' => current.borrow_mut().push('\t'),
            'r' => current.borrow_mut().push('\r'),
            _ => current.borrow_mut().push(c),
        }
        self.position += 1;
    }

    fn get_quoted_string(&mut self, current: Rc<RefCell<String>>) {
        println!("get_quoted_string");
        while self.position < self.input.len() {
            let c = self.input[self.position];
            self.position += 1;

            if c == '\'' {
                break;
            }

            current.borrow_mut().push(c);
        }
    }

    fn get_double_quoted_string(&mut self, current: Rc<RefCell<String>>) {
        while self.position < self.input.len() {
            let c = self.input[self.position];
            self.position += 1;

            if c == '"' {
                break;
            }
            current.borrow_mut().push(c);
        }
    }
}
