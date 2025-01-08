use super::*;

#[test]
fn lexer_echo_test_quoted() {
    let mut lexer = Lexer::new("'echo test'".to_string());

    assert_eq!(lexer.next(), Some(Token::new("echo test".to_string(), Vocabulary::Word)));
    assert_eq!(lexer.next(), None);
}

#[test]
fn lexer_echo_test_double_quoted() {
    let mut lexer = Lexer::new("\"echo test\"".to_string());

    assert_eq!(lexer.next(), Some(Token::new("echo test".to_string(), Vocabulary::Word)));
    assert_eq!(lexer.next(), None);
}

#[test]
fn lexer_echo_test_linked_quoted() {
    let mut lexer = Lexer::new("echo''test".to_string());

    assert_eq!(lexer.next(), Some(Token::new("echotest".to_string(), Vocabulary::Word)));
    assert_eq!(lexer.next(), None);
}

#[test]
fn lexer_echo_test_spaced_quoted() {
    let mut lexer = Lexer::new("echo ' test'".to_string());

    assert_eq!(lexer.next(), Some(Token::new("echo".to_string(), Vocabulary::Word)));
    assert_eq!(lexer.next(), Some(Token::new(" test".to_string(), Vocabulary::Word)));
    assert_eq!(lexer.next(), None);
}

#[test]
fn lexer_echo_test_piped_cat() {
    let mut lexer = Lexer::new("echo test | cat".to_string());

    assert_eq!(lexer.next(), Some(Token::new("echo".to_string(), Vocabulary::Word)));
    assert_eq!(lexer.next(), Some(Token::new("test".to_string(), Vocabulary::Word)));
    assert_eq!(lexer.next(), Some(Token::new("|".to_string(), Vocabulary::RPipe)));
    assert_eq!(lexer.next(), Some(Token::new("cat".to_string(), Vocabulary::Word)));
    assert_eq!(lexer.next(), None);
}

#[test]
fn lexer_echo_test_nospace_left_piped_cat() {
    let mut lexer = Lexer::new("echo test| cat".to_string());

    assert_eq!(lexer.next(), Some(Token::new("echo".to_string(), Vocabulary::Word)));
    assert_eq!(lexer.next(), Some(Token::new("test".to_string(), Vocabulary::Word)));
    assert_eq!(lexer.next(), Some(Token::new("|".to_string(), Vocabulary::RPipe)));
    assert_eq!(lexer.next(), Some(Token::new("cat".to_string(), Vocabulary::Word)));
    assert_eq!(lexer.next(), None);
}

#[test]
fn lexer_echo_test_nospace_right_piped_cat() {
    let mut lexer = Lexer::new("echo test |cat".to_string());

    assert_eq!(lexer.next(), Some(Token::new("echo".to_string(), Vocabulary::Word)));
    assert_eq!(lexer.next(), Some(Token::new("test".to_string(), Vocabulary::Word)));
    assert_eq!(lexer.next(), Some(Token::new("|".to_string(), Vocabulary::RPipe)));
    assert_eq!(lexer.next(), Some(Token::new("cat".to_string(), Vocabulary::Word)));
    assert_eq!(lexer.next(), None);
}

#[test]
fn lexer_echo_test_nospace_piped_cat() {
    let mut lexer = Lexer::new("echo test|cat".to_string());

    assert_eq!(lexer.next(), Some(Token::new("echo".to_string(), Vocabulary::Word)));
    assert_eq!(lexer.next(), Some(Token::new("test".to_string(), Vocabulary::Word)));
    assert_eq!(lexer.next(), Some(Token::new("|".to_string(), Vocabulary::RPipe)));
    assert_eq!(lexer.next(), Some(Token::new("cat".to_string(), Vocabulary::Word)));
    assert_eq!(lexer.next(), None);
}

#[test]
fn lexer_date_who_ls_cat() {
    let mut lexer = Lexer::new("date && who || ls; cat file".to_string());

    assert_eq!(lexer.next(), Some(Token::new("date".to_string(), Vocabulary::Word)));
    assert_eq!(lexer.next(), Some(Token::new("&&".to_string(), Vocabulary::And)));
    assert_eq!(lexer.next(), Some(Token::new("who".to_string(), Vocabulary::Word)));
    assert_eq!(lexer.next(), Some(Token::new("||".to_string(), Vocabulary::Or)));
    assert_eq!(lexer.next(), Some(Token::new("ls".to_string(), Vocabulary::Word)));
    assert_eq!(lexer.next(), Some(Token::new(";".to_string(), Vocabulary::Semicolon)));
    assert_eq!(lexer.next(), Some(Token::new("cat".to_string(), Vocabulary::Word)));
    assert_eq!(lexer.next(), Some(Token::new("file".to_string(), Vocabulary::Word)));
    assert_eq!(lexer.next(), None);
}

#[test]
fn lexer_date_who_ls_cat_nospace() {
    let mut lexer = Lexer::new("date&&who||ls;cat file".to_string());

    assert_eq!(lexer.next(), Some(Token::new("date".to_string(), Vocabulary::Word)));
    assert_eq!(lexer.next(), Some(Token::new("&&".to_string(), Vocabulary::And)));
    assert_eq!(lexer.next(), Some(Token::new("who".to_string(), Vocabulary::Word)));
    assert_eq!(lexer.next(), Some(Token::new("||".to_string(), Vocabulary::Or)));
    assert_eq!(lexer.next(), Some(Token::new("ls".to_string(), Vocabulary::Word)));
    assert_eq!(lexer.next(), Some(Token::new(";".to_string(), Vocabulary::Semicolon)));
    assert_eq!(lexer.next(), Some(Token::new("cat".to_string(), Vocabulary::Word)));
    assert_eq!(lexer.next(), Some(Token::new("file".to_string(), Vocabulary::Word)));
    assert_eq!(lexer.next(), None);
}
