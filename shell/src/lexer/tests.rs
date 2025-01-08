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
