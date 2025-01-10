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
fn lexer_echo_test_newline_test() {
    let mut lexer = Lexer::new("echo test \\\ntest".to_string());

    assert_eq!(lexer.next(), Some(Token::new("echo".to_string(), Vocabulary::Word)));
    assert_eq!(lexer.next(), Some(Token::new("test".to_string(), Vocabulary::Word)));
    assert_eq!(lexer.next(), Some(Token::new("test".to_string(), Vocabulary::Word)));
    assert_eq!(lexer.next(), None);
}

#[test]
fn lexer_echo_test_newline_test_quoted() {
    let mut lexer = Lexer::new("echo \"test \\\ntest\"".to_string());

    assert_eq!(lexer.next(), Some(Token::new("echo".to_string(), Vocabulary::Word)));
    assert_eq!(lexer.next(), Some(Token::new("test test".to_string(), Vocabulary::Word)));
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

#[test]
fn lexer_pipe_next_to_quote() {
    let mut lexer = Lexer::new("echo test \"\"| cat".to_string());

    assert_eq!(lexer.next(), Some(Token::new("echo".to_string(), Vocabulary::Word)));
    assert_eq!(lexer.next(), Some(Token::new("test".to_string(), Vocabulary::Word)));
    assert_eq!(lexer.next(), Some(Token::new("".to_string(), Vocabulary::Word)));
    assert_eq!(lexer.next(), Some(Token::new("|".to_string(), Vocabulary::RPipe)));
    assert_eq!(lexer.next(), Some(Token::new("cat".to_string(), Vocabulary::Word)));
    assert_eq!(lexer.next(), None);
}

#[test]
fn lexer_for_good() {
    let mut lexer = Lexer::new("for x in test1 test2; do echo x; done".to_string());

    assert_eq!(lexer.next(), Some(Token::new("for".to_string(), Vocabulary::For)));
    assert_eq!(lexer.next(), Some(Token::new("x".to_string(), Vocabulary::Word)));
    assert_eq!(lexer.next(), Some(Token::new("in".to_string(), Vocabulary::In)));
    assert_eq!(lexer.next(), Some(Token::new("test1".to_string(), Vocabulary::Word)));
    assert_eq!(lexer.next(), Some(Token::new("test2".to_string(), Vocabulary::Word)));
    assert_eq!(lexer.next(), Some(Token::new(";".to_string(), Vocabulary::Semicolon)));
    assert_eq!(lexer.next(), Some(Token::new("do".to_string(), Vocabulary::Do)));
    assert_eq!(lexer.next(), Some(Token::new("echo".to_string(), Vocabulary::Word)));
    assert_eq!(lexer.next(), Some(Token::new("x".to_string(), Vocabulary::Word)));
    assert_eq!(lexer.next(), Some(Token::new(";".to_string(), Vocabulary::Semicolon)));
    assert_eq!(lexer.next(), Some(Token::new("done".to_string(), Vocabulary::Done)));
    assert_eq!(lexer.next(), None);
}

#[test]
fn lexer_for_wrong_double_quoted() {
    let mut lexer = Lexer::new("\"\"for x in test1 test2; do echo x; done".to_string());

    assert_eq!(lexer.next(), Some(Token::new("for".to_string(), Vocabulary::Word)));
    assert_eq!(lexer.next(), Some(Token::new("x".to_string(), Vocabulary::Word)));
    assert_eq!(lexer.next(), Some(Token::new("in".to_string(), Vocabulary::In)));
    assert_eq!(lexer.next(), Some(Token::new("test1".to_string(), Vocabulary::Word)));
    assert_eq!(lexer.next(), Some(Token::new("test2".to_string(), Vocabulary::Word)));
    assert_eq!(lexer.next(), Some(Token::new(";".to_string(), Vocabulary::Semicolon)));
    assert_eq!(lexer.next(), Some(Token::new("do".to_string(), Vocabulary::Do)));
    assert_eq!(lexer.next(), Some(Token::new("echo".to_string(), Vocabulary::Word)));
    assert_eq!(lexer.next(), Some(Token::new("x".to_string(), Vocabulary::Word)));
    assert_eq!(lexer.next(), Some(Token::new(";".to_string(), Vocabulary::Semicolon)));
    assert_eq!(lexer.next(), Some(Token::new("done".to_string(), Vocabulary::Done)));
    assert_eq!(lexer.next(), None);
}
