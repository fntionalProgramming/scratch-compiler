use super::*;

// shorthand so each test reads as one line
fn token(token_type: TokenType, lexeme: &str, literal: Literal, line: usize) -> Token {
    Token::new(token_type, lexeme.to_string(), literal, line)
}

// ---------------- new ----------------

#[test]
fn new_stores_every_field() {
    let t = token(TokenType::Identifier, "foo", Literal::Nil, 7);
    assert_eq!(t.token_type, TokenType::Identifier);
    assert_eq!(t.lexeme, "foo");
    assert_eq!(t.literal, Literal::Nil);
    assert_eq!(t.line, 7);
}

#[test]
fn new_accepts_line_zero_and_max() {
    assert_eq!(token(TokenType::Eof, "", Literal::Nil, 0).line, 0);
    assert_eq!(token(TokenType::Eof, "", Literal::Nil, usize::MAX).line, usize::MAX);
}

#[test]
fn new_does_not_check_that_lexeme_matches_type() {
    // Token is plain data; checking that "+" really is a Plus is the scanner's job
    let t = token(TokenType::Plus, "not a plus", Literal::Number(1.0), 1);
    assert_eq!(t.lexeme(), "not a plus");
}

// ---------------- lexeme ----------------

#[test]
fn lexeme_returns_exact_text() {
    assert_eq!(token(TokenType::BangEqual, "!=", Literal::Nil, 1).lexeme(), "!=");
}

#[test]
fn lexeme_can_be_empty() {
    // EOF has no source text
    assert_eq!(token(TokenType::Eof, "", Literal::Nil, 1).lexeme(), "");
}

#[test]
fn lexeme_keeps_quotes_whitespace_and_unicode() {
    assert_eq!(token(TokenType::Str, "\"a b\"", Literal::Nil, 1).lexeme(), "\"a b\"");
    assert_eq!(token(TokenType::Str, "\"line1\nline2\"", Literal::Nil, 1).lexeme(), "\"line1\nline2\"");
    assert_eq!(token(TokenType::Identifier, "héllo_世界", Literal::Nil, 1).lexeme(), "héllo_世界");
}

// ---------------- Display ----------------

#[test]
fn display_single_character_token() {
    assert_eq!(token(TokenType::LeftParen, "(", Literal::Nil, 1).to_string(), "LEFT_PAREN ( nil");
}

#[test]
fn display_two_character_token() {
    assert_eq!(token(TokenType::GreaterEqual, ">=", Literal::Nil, 1).to_string(), "GREATER_EQUAL >= nil");
}

#[test]
fn display_keyword() {
    assert_eq!(token(TokenType::While, "while", Literal::Nil, 1).to_string(), "WHILE while nil");
}

#[test]
fn display_identifier() {
    assert_eq!(token(TokenType::Identifier, "count", Literal::Nil, 1).to_string(), "IDENTIFIER count nil");
}

#[test]
fn display_string_shows_lexeme_with_quotes_and_value_without() {
    let t = token(TokenType::Str, "\"hi\"", Literal::Str("hi".to_string()), 1);
    assert_eq!(t.to_string(), "STRING \"hi\" hi");
}

#[test]
fn display_empty_string_literal() {
    // the value is empty, so the line ends with a trailing space
    let t = token(TokenType::Str, "\"\"", Literal::Str(String::new()), 1);
    assert_eq!(t.to_string(), "STRING \"\" ");
}

#[test]
fn display_whole_number_has_no_decimal_point() {
    // Rust prints 123.0 as "123", the lexeme keeps what was typed
    let t = token(TokenType::Number, "123.0", Literal::Number(123.0), 1);
    assert_eq!(t.to_string(), "NUMBER 123.0 123");
}

#[test]
fn display_fractional_number() {
    let t = token(TokenType::Number, "45.67", Literal::Number(45.67), 1);
    assert_eq!(t.to_string(), "NUMBER 45.67 45.67");
}

#[test]
fn display_zero_and_negative_zero() {
    assert_eq!(token(TokenType::Number, "0", Literal::Number(0.0), 1).to_string(), "NUMBER 0 0");
    assert_eq!(token(TokenType::Number, "0", Literal::Number(-0.0), 1).to_string(), "NUMBER 0 -0");
}

#[test]
fn display_very_large_number_is_written_out_in_full() {
    // f64 Display never uses scientific notation
    let t = token(TokenType::Number, "1e21", Literal::Number(1e21), 1);
    assert_eq!(t.to_string(), "NUMBER 1e21 1000000000000000000000");
}

#[test]
fn display_true_and_false() {
    assert_eq!(token(TokenType::True, "true", Literal::Bool(true), 1).to_string(), "TRUE true true");
    assert_eq!(token(TokenType::False, "false", Literal::Bool(false), 1).to_string(), "FALSE false false");
}

#[test]
fn display_nil_keyword() {
    assert_eq!(token(TokenType::Nil, "nil", Literal::Nil, 1).to_string(), "NIL nil nil");
}

#[test]
fn display_eof_has_two_spaces_for_empty_lexeme() {
    assert_eq!(token(TokenType::Eof, "", Literal::Nil, 1).to_string(), "EOF  nil");
}

#[test]
fn display_does_not_include_line() {
    let a = token(TokenType::Dot, ".", Literal::Nil, 1);
    let b = token(TokenType::Dot, ".", Literal::Nil, 999);
    assert_eq!(a.to_string(), b.to_string());
}

#[test]
fn display_ignores_width() {
    // write! inside fmt doesn't pass the width on, so {:>30} has no effect
    let t = token(TokenType::Dot, ".", Literal::Nil, 1);
    assert_eq!(format!("{:>30}", t), "DOT . nil");
}

#[test]
fn display_every_token_type_name() {
    let cases = [
        (TokenType::LeftParen, "LEFT_PAREN"), (TokenType::RightParen, "RIGHT_PAREN"),
        (TokenType::LeftBrace, "LEFT_BRACE"), (TokenType::RightBrace, "RIGHT_BRACE"),
        (TokenType::Comma, "COMMA"), (TokenType::Dot, "DOT"), (TokenType::Minus, "MINUS"),
        (TokenType::Plus, "PLUS"), (TokenType::Semicolon, "SEMICOLON"), (TokenType::Slash, "SLASH"),
        (TokenType::Star, "STAR"), (TokenType::Bang, "BANG"), (TokenType::BangEqual, "BANG_EQUAL"),
        (TokenType::Equal, "EQUAL"), (TokenType::EqualEqual, "EQUAL_EQUAL"),
        (TokenType::Greater, "GREATER"), (TokenType::GreaterEqual, "GREATER_EQUAL"),
        (TokenType::Less, "LESS"), (TokenType::LessEqual, "LESS_EQUAL"),
        (TokenType::Identifier, "IDENTIFIER"), (TokenType::Str, "STRING"), (TokenType::Number, "NUMBER"),
        (TokenType::And, "AND"), (TokenType::Class, "CLASS"), (TokenType::Else, "ELSE"),
        (TokenType::False, "FALSE"), (TokenType::Fun, "FUN"), (TokenType::For, "FOR"),
        (TokenType::If, "IF"), (TokenType::Nil, "NIL"), (TokenType::Or, "OR"),
        (TokenType::Print, "PRINT"), (TokenType::Return, "RETURN"), (TokenType::Super, "SUPER"),
        (TokenType::This, "THIS"), (TokenType::True, "TRUE"), (TokenType::Var, "VAR"),
        (TokenType::While, "WHILE"), (TokenType::Eof, "EOF"),
    ];
    for (token_type, name) in cases {
        let t = token(token_type, "x", Literal::Nil, 1);
        assert_eq!(t.to_string(), format!("{} x nil", name));
    }
}

// ---------------- PartialEq / Clone ----------------

#[test]
fn equal_when_every_field_matches() {
    let a = token(TokenType::Number, "1", Literal::Number(1.0), 3);
    let b = token(TokenType::Number, "1", Literal::Number(1.0), 3);
    assert_eq!(a, b);
}

#[test]
fn not_equal_when_any_single_field_differs() {
    let base = token(TokenType::Number, "1", Literal::Number(1.0), 3);
    assert_ne!(base, token(TokenType::Identifier, "1", Literal::Number(1.0), 3));
    assert_ne!(base, token(TokenType::Number, "1.0", Literal::Number(1.0), 3));
    assert_ne!(base, token(TokenType::Number, "1", Literal::Number(2.0), 3));
    assert_ne!(base, token(TokenType::Number, "1", Literal::Number(1.0), 4));
}

#[test]
fn nan_token_is_not_equal_to_itself() {
    // f64 NaN != NaN, and PartialEq passes that through
    let t = token(TokenType::Number, "nan", Literal::Number(f64::NAN), 1);
    assert_ne!(t, t.clone());
}

#[test]
fn clone_is_equal_and_independent() {
    let original = token(TokenType::Str, "\"a\"", Literal::Str("a".to_string()), 2);
    let mut copy = original.clone();
    assert_eq!(original, copy);
    copy.lexeme.push('!');
    assert_eq!(original.lexeme(), "\"a\"");
}

#[test]
fn debug_output_names_the_struct() {
    let t = token(TokenType::Dot, ".", Literal::Nil, 1);
    assert!(format!("{:?}", t).starts_with("Token {"));
}
