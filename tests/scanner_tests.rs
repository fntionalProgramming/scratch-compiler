use super::*;

// ---------------- helpers ----------------

fn scan(source: &str) -> Result<Vec<Token>, Vec<ScanError>> {
    Scanner::new(source).scan_tokens()
}

fn tok(token_type: TokenType, lexeme: &str, literal: Literal, line: usize) -> Token {
    Token::new(token_type, lexeme.to_string(), literal, line)
}

fn eof(line: usize) -> Token {
    tok(TokenType::Eof, "", Literal::Nil, line)
}

// scans and returns only the token types, for tests that don't care about the rest
fn types(source: &str) -> Vec<TokenType> {
    let mut result = Vec::new();
    for token in scan(source).expect("expected no scan errors") {
        result.push(token_type_of(&token));
    }
    result
}

// Token's fields are private to token.rs, so read the type back from its Display
fn token_type_of(token: &Token) -> TokenType {
    let text = token.to_string();
    let name = text.split(' ').next().unwrap().to_string();
    let all = [
        TokenType::LeftParen, TokenType::RightParen, TokenType::LeftBrace, TokenType::RightBrace,
        TokenType::Comma, TokenType::Dot, TokenType::Minus, TokenType::Plus, TokenType::Semicolon,
        TokenType::Slash, TokenType::Star, TokenType::Bang, TokenType::BangEqual, TokenType::Equal,
        TokenType::EqualEqual, TokenType::Greater, TokenType::GreaterEqual, TokenType::Less,
        TokenType::LessEqual, TokenType::Identifier, TokenType::Str, TokenType::Number,
        TokenType::And, TokenType::Class, TokenType::Else, TokenType::False, TokenType::Fun,
        TokenType::For, TokenType::If, TokenType::Nil, TokenType::Or, TokenType::Print,
        TokenType::Return, TokenType::Super, TokenType::This, TokenType::True, TokenType::Var,
        TokenType::While, TokenType::Eof,
    ];
    for t in all {
        if t.to_string() == name {
            return t;
        }
    }
    panic!("unknown token type {name}");
}

fn err(kind: ScanErrorKind, line: usize, column: usize) -> ScanError {
    ScanError::new(kind, line, column)
}

// ---------------- empty input and whitespace ----------------

#[test]
fn empty_source_gives_only_eof() {
    assert_eq!(scan(""), Ok(vec![eof(1)]));
}

#[test]
fn whitespace_only_gives_only_eof() {
    assert_eq!(scan(" \t\r "), Ok(vec![eof(1)]));
}

#[test]
fn eof_line_counts_every_newline() {
    assert_eq!(scan("\n\n\n"), Ok(vec![eof(4)]));
}

#[test]
fn windows_line_endings_count_once() {
    assert_eq!(scan("a\r\nb"), Ok(vec![
        tok(TokenType::Identifier, "a", Literal::Nil, 1),
        tok(TokenType::Identifier, "b", Literal::Nil, 2),
        eof(2),
    ]));
}

// ---------------- single characters ----------------

#[test]
fn every_single_character_token() {
    use TokenType::*;
    assert_eq!(
        types("(){},.-+;*/"),
        vec![LeftParen, RightParen, LeftBrace, RightBrace, Comma, Dot, Minus, Plus, Semicolon, Star, Slash, Eof]
    );
}

#[test]
fn single_character_token_has_its_lexeme_and_nil() {
    assert_eq!(scan("+"), Ok(vec![tok(TokenType::Plus, "+", Literal::Nil, 1), eof(1)]));
}

// ---------------- one or two characters ----------------

#[test]
fn every_two_character_token() {
    use TokenType::*;
    assert_eq!(types("!= == <= >="), vec![BangEqual, EqualEqual, LessEqual, GreaterEqual, Eof]);
}

#[test]
fn every_one_character_form_of_those() {
    use TokenType::*;
    assert_eq!(types("! = < >"), vec![Bang, Equal, Less, Greater, Eof]);
}

#[test]
fn two_character_token_keeps_both_characters() {
    assert_eq!(scan(">="), Ok(vec![tok(TokenType::GreaterEqual, ">=", Literal::Nil, 1), eof(1)]));
}

#[test]
fn longest_match_wins() {
    use TokenType::*;
    // "!==" is != then =, not ! then ==
    assert_eq!(types("!=="), vec![BangEqual, Equal, Eof]);
    assert_eq!(types("==="), vec![EqualEqual, Equal, Eof]);
    assert_eq!(types("<=="), vec![LessEqual, Equal, Eof]);
}

#[test]
fn space_splits_a_two_character_token() {
    use TokenType::*;
    assert_eq!(types("! ="), vec![Bang, Equal, Eof]);
}

#[test]
fn operator_at_very_end_of_source() {
    use TokenType::*;
    assert_eq!(types("a <"), vec![Identifier, Less, Eof]);
}

// ---------------- comments ----------------

#[test]
fn comment_is_skipped() {
    assert_eq!(scan("// hello"), Ok(vec![eof(1)]));
}

#[test]
fn comment_ends_at_newline() {
    assert_eq!(scan("// hello\n+"), Ok(vec![tok(TokenType::Plus, "+", Literal::Nil, 2), eof(2)]));
}

#[test]
fn comment_after_code() {
    use TokenType::*;
    assert_eq!(types("1 + 2 // sum"), vec![Number, Plus, Number, Eof]);
}

#[test]
fn comment_can_hold_any_character() {
    // characters that would be errors outside a comment are fine inside one
    assert_eq!(scan("// @#$ \"unterminated é 世界"), Ok(vec![eof(1)]));
}

#[test]
fn slash_alone_is_slash() {
    use TokenType::*;
    assert_eq!(types("a / b"), vec![Identifier, Slash, Identifier, Eof]);
}

#[test]
fn slash_space_slash_is_two_slashes() {
    use TokenType::*;
    assert_eq!(types("/ /"), vec![Slash, Slash, Eof]);
}

#[test]
fn three_slashes_is_still_a_comment() {
    assert_eq!(scan("///"), Ok(vec![eof(1)]));
}

// ---------------- strings ----------------

#[test]
fn string_value_has_no_quotes_but_lexeme_does() {
    assert_eq!(scan("\"hi\""), Ok(vec![
        tok(TokenType::Str, "\"hi\"", Literal::Str("hi".to_string()), 1),
        eof(1),
    ]));
}

#[test]
fn empty_string() {
    assert_eq!(scan("\"\""), Ok(vec![
        tok(TokenType::Str, "\"\"", Literal::Str(String::new()), 1),
        eof(1),
    ]));
}

#[test]
fn string_keeps_spaces_and_symbols() {
    assert_eq!(scan("\" a // b + \""), Ok(vec![
        tok(TokenType::Str, "\" a // b + \"", Literal::Str(" a // b + ".to_string()), 1),
        eof(1),
    ]));
}

#[test]
fn string_has_no_escape_sequences() {
    // Lox has no escapes, so \n stays as a backslash and an n
    assert_eq!(scan(r#""a\nb""#), Ok(vec![
        tok(TokenType::Str, r#""a\nb""#, Literal::Str(r"a\nb".to_string()), 1),
        eof(1),
    ]));
}

#[test]
fn multi_line_string_is_on_its_starting_line() {
    assert_eq!(scan("\"a\nb\"\n+"), Ok(vec![
        tok(TokenType::Str, "\"a\nb\"", Literal::Str("a\nb".to_string()), 1),
        tok(TokenType::Plus, "+", Literal::Nil, 3),
        eof(3),
    ]));
}

#[test]
fn string_with_unicode() {
    assert_eq!(scan("\"héllo 世界 🎉\""), Ok(vec![
        tok(TokenType::Str, "\"héllo 世界 🎉\"", Literal::Str("héllo 世界 🎉".to_string()), 1),
        eof(1),
    ]));
}

#[test]
fn two_strings_next_to_each_other() {
    use TokenType::*;
    assert_eq!(types("\"a\"\"b\""), vec![Str, Str, Eof]);
}

#[test]
fn unterminated_string_is_an_error() {
    assert_eq!(scan("\"abc"), Err(vec![err(ScanErrorKind::UnterminatedString, 1, 1)]));
}

#[test]
fn lone_quote_is_unterminated() {
    assert_eq!(scan("\""), Err(vec![err(ScanErrorKind::UnterminatedString, 1, 1)]));
}

#[test]
fn unterminated_string_error_points_at_opening_quote() {
    // reported where the string starts, even though it runs over two lines
    assert_eq!(scan("x = \"abc\ndef"), Err(vec![err(ScanErrorKind::UnterminatedString, 1, 5)]));
}

// ---------------- numbers ----------------

#[test]
fn integer() {
    assert_eq!(scan("123"), Ok(vec![
        tok(TokenType::Number, "123", Literal::Number(123.0), 1),
        eof(1),
    ]));
}

#[test]
fn zero() {
    assert_eq!(scan("0"), Ok(vec![tok(TokenType::Number, "0", Literal::Number(0.0), 1), eof(1)]));
}

#[test]
fn leading_zeros_are_allowed() {
    assert_eq!(scan("007"), Ok(vec![tok(TokenType::Number, "007", Literal::Number(7.0), 1), eof(1)]));
}

#[test]
fn decimal() {
    assert_eq!(scan("45.67"), Ok(vec![
        tok(TokenType::Number, "45.67", Literal::Number(45.67), 1),
        eof(1),
    ]));
}

#[test]
fn decimals_match_rusts_own_parsing() {
    // the hand-written conversion should round exactly like str::parse
    let cases = ["0.1", "0.2", "0.3", "1.5", "3.14159", "123.456", "0.000001", "99999.99999"];
    for text in cases {
        let expected: f64 = text.parse().unwrap();
        assert_eq!(
            scan(text),
            Ok(vec![tok(TokenType::Number, text, Literal::Number(expected), 1), eof(1)]),
            "for {text}"
        );
    }
}

#[test]
fn trailing_dot_is_not_part_of_the_number() {
    use TokenType::*;
    assert_eq!(types("123."), vec![Number, Dot, Eof]);
}

#[test]
fn leading_dot_is_not_part_of_the_number() {
    use TokenType::*;
    assert_eq!(types(".5"), vec![Dot, Number, Eof]);
}

#[test]
fn second_dot_starts_a_new_token() {
    use TokenType::*;
    assert_eq!(types("1.2.3"), vec![Number, Dot, Number, Eof]);
}

#[test]
fn method_call_on_a_number() {
    use TokenType::*;
    assert_eq!(types("1.abs"), vec![Number, Dot, Identifier, Eof]);
}

#[test]
fn negative_number_is_minus_then_number() {
    use TokenType::*;
    assert_eq!(types("-5"), vec![Minus, Number, Eof]);
}

#[test]
fn number_then_letters_splits() {
    use TokenType::*;
    assert_eq!(types("1abc"), vec![Number, Identifier, Eof]);
}

#[test]
fn large_number_that_still_fits() {
    let text = "1".to_string() + &"0".repeat(300);
    let expected: f64 = text.parse().unwrap();
    assert_eq!(scan(&text), Ok(vec![tok(TokenType::Number, &text, Literal::Number(expected), 1), eof(1)]));
}

#[test]
fn number_too_large_for_f64_is_an_error() {
    let text = "1".to_string() + &"0".repeat(400);
    assert_eq!(scan(&text), Err(vec![err(ScanErrorKind::NumberTooLarge, 1, 1)]));
}

#[test]
fn very_long_fraction_is_fine() {
    let text = "1.".to_string() + &"5".repeat(400);
    let expected: f64 = text.parse().unwrap();
    assert_eq!(scan(&text), Ok(vec![tok(TokenType::Number, &text, Literal::Number(expected), 1), eof(1)]));
}

// ---------------- identifiers and keywords ----------------

#[test]
fn identifier() {
    assert_eq!(scan("foo"), Ok(vec![tok(TokenType::Identifier, "foo", Literal::Nil, 1), eof(1)]));
}

#[test]
fn identifier_with_underscores_and_digits() {
    use TokenType::*;
    assert_eq!(types("_ _a a_1 A1_b2"), vec![Identifier, Identifier, Identifier, Identifier, Eof]);
}

#[test]
fn every_keyword() {
    use TokenType::*;
    assert_eq!(
        types("and class else false for fun if nil or print return super this true var while"),
        vec![And, Class, Else, False, For, Fun, If, Nil, Or, Print, Return, Super, This, True, Var, While, Eof]
    );
}

#[test]
fn true_and_false_carry_bool_values() {
    assert_eq!(scan("true false"), Ok(vec![
        tok(TokenType::True, "true", Literal::Bool(true), 1),
        tok(TokenType::False, "false", Literal::Bool(false), 1),
        eof(1),
    ]));
}

#[test]
fn other_keywords_carry_nil() {
    assert_eq!(scan("nil"), Ok(vec![tok(TokenType::Nil, "nil", Literal::Nil, 1), eof(1)]));
}

#[test]
fn keywords_are_case_sensitive() {
    use TokenType::*;
    assert_eq!(types("And CLASS True"), vec![Identifier, Identifier, Identifier, Eof]);
}

#[test]
fn keyword_inside_longer_word_is_identifier() {
    use TokenType::*;
    assert_eq!(types("orchid android classy iffy"), vec![Identifier, Identifier, Identifier, Identifier, Eof]);
}

#[test]
fn keyword_with_suffix_is_identifier() {
    use TokenType::*;
    assert_eq!(types("var_ while1"), vec![Identifier, Identifier, Eof]);
}

// ---------------- unexpected characters ----------------

#[test]
fn unexpected_ascii_character() {
    assert_eq!(scan("@"), Err(vec![err(ScanErrorKind::UnexpectedCharacter('@'), 1, 1)]));
}

#[test]
fn every_unexpected_ascii_symbol() {
    for c in ['@', '#', '$', '%', '^', '&', '|', '~', '`', '?', ':', '[', ']', '\\', '\''] {
        assert_eq!(
            scan(&c.to_string()),
            Err(vec![err(ScanErrorKind::UnexpectedCharacter(c), 1, 1)]),
            "for {c:?}"
        );
    }
}

#[test]
fn unicode_letter_is_one_error_not_several() {
    // é is two bytes, but should be reported once as one character
    assert_eq!(scan("é"), Err(vec![err(ScanErrorKind::UnexpectedCharacter('é'), 1, 1)]));
}

#[test]
fn emoji_is_one_error() {
    assert_eq!(scan("🎉"), Err(vec![err(ScanErrorKind::UnexpectedCharacter('🎉'), 1, 1)]));
}

#[test]
fn unicode_inside_identifier_splits_it() {
    assert_eq!(scan("caf\u{e9}"), Err(vec![err(ScanErrorKind::UnexpectedCharacter('é'), 1, 4)]));
}

#[test]
fn null_byte_is_unexpected() {
    assert_eq!(scan("\0"), Err(vec![err(ScanErrorKind::UnexpectedCharacter('\0'), 1, 1)]));
}

// ---------------- error positions and collecting ----------------

#[test]
fn error_column_counts_from_one() {
    assert_eq!(scan("ab @"), Err(vec![err(ScanErrorKind::UnexpectedCharacter('@'), 1, 4)]));
}

#[test]
fn error_column_resets_on_new_line() {
    assert_eq!(scan("abc\n  @"), Err(vec![err(ScanErrorKind::UnexpectedCharacter('@'), 2, 3)]));
}

#[test]
fn error_column_counts_characters_not_bytes() {
    // the string before @ has multi-byte characters, but each counts as one column
    assert_eq!(scan("\"é世\" @"), Err(vec![err(ScanErrorKind::UnexpectedCharacter('@'), 1, 6)]));
}

#[test]
fn error_after_multi_line_string_uses_the_right_line() {
    assert_eq!(scan("\"a\nbc\" @"), Err(vec![err(ScanErrorKind::UnexpectedCharacter('@'), 2, 5)]));
}

#[test]
fn every_error_is_reported_in_order() {
    assert_eq!(scan("@ # \n $"), Err(vec![
        err(ScanErrorKind::UnexpectedCharacter('@'), 1, 1),
        err(ScanErrorKind::UnexpectedCharacter('#'), 1, 3),
        err(ScanErrorKind::UnexpectedCharacter('$'), 2, 2),
    ]));
}

#[test]
fn scanning_keeps_going_after_an_error() {
    // the unterminated string still gets reported after the bad character
    assert_eq!(scan("@ \"abc"), Err(vec![
        err(ScanErrorKind::UnexpectedCharacter('@'), 1, 1),
        err(ScanErrorKind::UnterminatedString, 1, 3),
    ]));
}

#[test]
fn different_error_kinds_together() {
    let big = "9".repeat(400);
    let source = format!("@\n{big}\n\"x");
    assert_eq!(scan(&source), Err(vec![
        err(ScanErrorKind::UnexpectedCharacter('@'), 1, 1),
        err(ScanErrorKind::NumberTooLarge, 2, 1),
        err(ScanErrorKind::UnterminatedString, 3, 1),
    ]));
}

// ---------------- whole programs ----------------

#[test]
fn small_program() {
    let source = "var x = 1;\nprint x + 2.5; // done\n";
    assert_eq!(scan(source), Ok(vec![
        tok(TokenType::Var, "var", Literal::Nil, 1),
        tok(TokenType::Identifier, "x", Literal::Nil, 1),
        tok(TokenType::Equal, "=", Literal::Nil, 1),
        tok(TokenType::Number, "1", Literal::Number(1.0), 1),
        tok(TokenType::Semicolon, ";", Literal::Nil, 1),
        tok(TokenType::Print, "print", Literal::Nil, 2),
        tok(TokenType::Identifier, "x", Literal::Nil, 2),
        tok(TokenType::Plus, "+", Literal::Nil, 2),
        tok(TokenType::Number, "2.5", Literal::Number(2.5), 2),
        tok(TokenType::Semicolon, ";", Literal::Nil, 2),
        eof(3),
    ]));
}

#[test]
fn no_spaces_between_tokens() {
    use TokenType::*;
    assert_eq!(
        types("if(x>=1){print\"ok\";}"),
        vec![If, LeftParen, Identifier, GreaterEqual, Number, RightParen, LeftBrace, Print, Str, Semicolon, RightBrace, Eof]
    );
}

#[test]
fn scanning_twice_with_one_scanner_gives_only_eof() {
    // the first call uses up the source; a second call has nothing left to scan
    let mut scanner = Scanner::new("1 + 2");
    assert!(scanner.scan_tokens().is_ok());
    assert_eq!(scanner.scan_tokens(), Ok(vec![eof(1)]));
}

// ---------------- safe_convert_str_to_float ----------------

#[test]
fn convert_plain_numbers() {
    assert_eq!(safe_convert_str_to_float("0"), Some(0.0));
    assert_eq!(safe_convert_str_to_float("42"), Some(42.0));
    assert_eq!(safe_convert_str_to_float("2.5"), Some(2.5));
}

#[test]
fn convert_rejects_non_digits() {
    assert_eq!(safe_convert_str_to_float("1a"), None);
    assert_eq!(safe_convert_str_to_float("-1"), None);
    assert_eq!(safe_convert_str_to_float("1e5"), None);
    assert_eq!(safe_convert_str_to_float(" 1"), None);
}

#[test]
fn convert_rejects_two_dots() {
    assert_eq!(safe_convert_str_to_float("1.2.3"), None);
}

#[test]
fn convert_edge_shapes() {
    // the scanner never passes these, but the function still has to answer something
    // no digits at all isn't a number
    assert_eq!(safe_convert_str_to_float(""), None);
    assert_eq!(safe_convert_str_to_float("."), None);
    assert_eq!(safe_convert_str_to_float("5."), Some(5.0));
    assert_eq!(safe_convert_str_to_float(".5"), Some(0.5));
}

#[test]
fn convert_largest_and_too_large() {
    let max = format!("{}", f64::MAX);
    assert_eq!(safe_convert_str_to_float(&max), Some(f64::MAX));
    assert_eq!(safe_convert_str_to_float(&("1".to_string() + &"0".repeat(309))), None);
}

#[test]
fn convert_tiny_fraction() {
    let text = "0.".to_string() + &"0".repeat(30) + "1";
    let expected: f64 = text.parse().unwrap();
    assert_eq!(safe_convert_str_to_float(&text), Some(expected));
}
