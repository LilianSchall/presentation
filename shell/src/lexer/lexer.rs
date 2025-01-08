use super::token::Token;
use super::vocab::{generate_mapper, generate_separators, Vocabulary};
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

pub struct Lexer {
    pub mapper: HashMap<String, Vocabulary>,
    pub separators: Vec<char>,
    pub position: usize,
    pub len: usize,
    pub input: Vec<char>,
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position >= self.len {
            return None;
        }

        let current: Rc<RefCell<String>> = Rc::new(RefCell::new(String::new()));
        let mut c = self.input[self.position];
        while self.position < self.len && self.is_whitespace(c) {
            self.position += 1;
            c = self.input[self.position];
        }

        // operators handling
        while self.position < self.len
            && (self.separators.contains(&c) || current.borrow().len() > 0)
        {
            self.position += 1;
            match self.craft_operator(Rc::clone(&current), c) {
                Some(v) => return Some(Token::new(current.borrow().clone(), v)),
                None => (),
            }
            c = self.input[self.position];
        }

        while self.position < self.len && !self.separators.contains(&c) {
            self.position += 1;

            // quoting and miscellaneous handling
            match c {
                '\\' => self.get_escaped_char(Rc::clone(&current)),
                '\'' => self.get_quoted_string(Rc::clone(&current)),
                '"' => self.get_double_quoted_string(Rc::clone(&current)),
                '#' => self.skip_comment(),
                _ => current.borrow_mut().push(c),
            }
            c = self.input[self.position];
        }

        while self.position < self.len && self.is_whitespace(self.input[self.position]) {
            self.position += 1;
        }

        let result: String = current.borrow().clone();

        match self.mapper.get(&result) {
            None => Some(Token::new(result, Vocabulary::Word)),
            Some(v) => return Some(Token::new(result, *v)),
        }
    }
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mapper = generate_mapper();
        let separators = generate_separators();
        let mut vec: Vec<char> = input.chars().collect();
        vec.push('\0');
        Lexer {
            mapper,
            separators,
            position: 0,
            len: vec.len() - 1,
            input: vec,
        }
    }

    fn is_whitespace(&self, c: char) -> bool {
        c == ' ' || c == '\t' || c == '\n' || c == '\r'
    }

    fn get_escaped_char(&mut self, current: Rc<RefCell<String>>) {
        if self.position >= self.len {
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
        while self.position < self.len {
            let c = self.input[self.position];
            self.position += 1;

            if c == '\'' {
                break;
            }

            current.borrow_mut().push(c);
        }
    }

    fn get_double_quoted_string(&mut self, current: Rc<RefCell<String>>) {
        while self.position < self.len {
            let c = self.input[self.position];
            self.position += 1;

            if c == '"' {
                break;
            }
            current.borrow_mut().push(c);
        }
    }

    fn skip_comment(&mut self) {
        while self.position < self.input.len() && self.input[self.position] != '\n' {
            self.position += 1;
        }
    }

    fn craft_operator(&mut self, current: Rc<RefCell<String>>, c: char) -> Option<Vocabulary> {
        let mut borrowed = current.borrow_mut();
        let borrow: String = borrowed.clone();
        let mut result: String = borrowed.clone();
        result.push(c);

        let option = match self.mapper.get(&result) {
            None => {
                self.position -= 1;
                match self.mapper.get(&borrow) {
                    None => panic!("Unknown token: {}", borrow),
                    Some(v) => Some(*v),
                }
            }
            _ => None,
        };

        if option.is_none() {
            borrowed.push(c);
            return None;
        }
        option
    }
}
