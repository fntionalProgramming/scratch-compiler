use super::*;

// ---------------- helpers ----------------

fn op(token_type: TokenType, lexeme: &str) -> Token {
    Token::new(token_type, lexeme.to_string(), Literal::Nil, 1)
}

fn num(n: f64) -> Expr {
    Expr::Literal(Literal::Number(n))
}

fn string(s: &str) -> Expr {
    Expr::Literal(Literal::Str(s.to_string()))
}

fn unary(token_type: TokenType, lexeme: &str, right: Expr) -> Expr {
    Expr::Unary(Unary { oeprator: op(token_type, lexeme), right: Box::new(right) })
}

fn binary(left: Expr, token_type: TokenType, lexeme: &str, right: Expr) -> Expr {
    Expr::Binary(Binary { left: Box::new(left), operator: op(token_type, lexeme), right: Box::new(right) })
}

fn group(middle: Expr) -> Expr {
    Expr::Grouping(Grouping { middle: Box::new(middle) })
}

fn print(expr: &Expr) -> String {
    match AstPrinter::new(expr) {
        Ok(printer) => printer.to_string(),
        Err(error) => panic!("printing failed: {error}"),
    }
}

// ---------------- literals ----------------

#[test]
fn whole_number_has_no_decimal_point() {
    assert_eq!(print(&num(123.0)), "123");
}

#[test]
fn fractional_number() {
    assert_eq!(print(&num(45.67)), "45.67");
}

#[test]
fn zero_and_negative_zero() {
    assert_eq!(print(&num(0.0)), "0");
    assert_eq!(print(&num(-0.0)), "-0");
}

#[test]
fn string_is_printed_without_quotes() {
    assert_eq!(print(&string("hello")), "hello");
}

#[test]
fn empty_string_prints_nothing() {
    assert_eq!(print(&string("")), "");
}

#[test]
fn string_with_spaces_and_parentheses_is_not_escaped() {
    // the printer doesn't quote strings, so they can look like structure
    assert_eq!(print(&string("(a b)")), "(a b)");
}

#[test]
fn booleans_and_nil() {
    assert_eq!(print(&Expr::Literal(Literal::Bool(true))), "true");
    assert_eq!(print(&Expr::Literal(Literal::Bool(false))), "false");
    assert_eq!(print(&Expr::Literal(Literal::Nil)), "nil");
}

// ---------------- unary ----------------

#[test]
fn negation() {
    assert_eq!(print(&unary(TokenType::Minus, "-", num(5.0))), "(- 5)");
}

#[test]
fn logical_not() {
    assert_eq!(print(&unary(TokenType::Bang, "!", Expr::Literal(Literal::Bool(true)))), "(! true)");
}

#[test]
fn double_negation() {
    let expr = unary(TokenType::Minus, "-", unary(TokenType::Minus, "-", num(1.0)));
    assert_eq!(print(&expr), "(- (- 1))");
}

#[test]
fn not_not() {
    let expr = unary(TokenType::Bang, "!", unary(TokenType::Bang, "!", Expr::Literal(Literal::Nil)));
    assert_eq!(print(&expr), "(! (! nil))");
}

// ---------------- grouping ----------------

#[test]
fn grouping_of_literal() {
    assert_eq!(print(&group(num(1.0))), "(group 1)");
}

#[test]
fn nested_groupings() {
    assert_eq!(print(&group(group(group(num(1.0))))), "(group (group (group 1)))");
}

#[test]
fn grouping_of_empty_string_leaves_trailing_space() {
    // the space before the part is written even when the part is empty
    assert_eq!(print(&group(string(""))), "(group )");
}

// ---------------- binary ----------------

#[test]
fn every_binary_operator() {
    let cases = [
        (TokenType::Plus, "+"), (TokenType::Minus, "-"), (TokenType::Star, "*"), (TokenType::Slash, "/"),
        (TokenType::EqualEqual, "=="), (TokenType::BangEqual, "!="),
        (TokenType::Less, "<"), (TokenType::LessEqual, "<="),
        (TokenType::Greater, ">"), (TokenType::GreaterEqual, ">="),
    ];
    for (token_type, lexeme) in cases {
        let expr = binary(num(1.0), token_type, lexeme, num(2.0));
        assert_eq!(print(&expr), format!("({lexeme} 1 2)"), "for {lexeme}");
    }
}

#[test]
fn left_leaning_chain() {
    // (1 + 2) + 3
    let expr = binary(binary(num(1.0), TokenType::Plus, "+", num(2.0)), TokenType::Plus, "+", num(3.0));
    assert_eq!(print(&expr), "(+ (+ 1 2) 3)");
}

#[test]
fn right_leaning_chain() {
    // 1 + (2 + 3), without a grouping node
    let expr = binary(num(1.0), TokenType::Plus, "+", binary(num(2.0), TokenType::Plus, "+", num(3.0)));
    assert_eq!(print(&expr), "(+ 1 (+ 2 3))");
}

#[test]
fn binary_of_mixed_literals() {
    let expr = binary(string("a"), TokenType::EqualEqual, "==", Expr::Literal(Literal::Nil));
    assert_eq!(print(&expr), "(== a nil)");
}

#[test]
fn operator_comes_from_the_lexeme_not_the_type() {
    // the printer trusts the token's text; checking it matches the type is the parser's job
    let expr = binary(num(1.0), TokenType::Plus, "plus", num(2.0));
    assert_eq!(print(&expr), "(plus 1 2)");
}

// ---------------- whole expressions ----------------

#[test]
fn book_example() {
    // -123 * (45.67)
    let expr = binary(
        unary(TokenType::Minus, "-", num(123.0)),
        TokenType::Star, "*",
        group(num(45.67)),
    );
    assert_eq!(print(&expr), "(* (- 123) (group 45.67))");
}

#[test]
fn every_kind_of_node_together() {
    // !(1 < 2) == (-3 >= "x")
    let expr = binary(
        unary(TokenType::Bang, "!", group(binary(num(1.0), TokenType::Less, "<", num(2.0)))),
        TokenType::EqualEqual, "==",
        group(binary(unary(TokenType::Minus, "-", num(3.0)), TokenType::GreaterEqual, ">=", string("x"))),
    );
    assert_eq!(print(&expr), "(== (! (group (< 1 2))) (group (>= (- 3) x)))");
}

#[test]
fn deep_nesting() {
    let mut expr = num(0.0);
    for _ in 0..200 {
        expr = group(expr);
    }
    let expected = "(group ".repeat(200) + "0" + &")".repeat(200);
    assert_eq!(print(&expr), expected);
}

// ---------------- capacity ----------------

#[test]
fn capacity_matches_length_exactly() {
    let expr = binary(unary(TokenType::Minus, "-", num(123.0)), TokenType::Star, "*", group(num(45.67)));
    let printer = AstPrinter::new(&expr).unwrap();
    assert_eq!(printer.text.capacity(), printer.text.len());
}

#[test]
fn capacity_counts_bytes_for_unicode() {
    // é and 世 are more than one byte each, and capacity is in bytes
    let expr = binary(string("é"), TokenType::Plus, "+", string("世界"));
    let printer = AstPrinter::new(&expr).unwrap();
    assert_eq!(printer.text, "(+ é 世界)");
    assert_eq!(printer.text.capacity(), printer.text.len());
}

#[test]
fn literal_alone_fits_in_its_capacity() {
    // a bare literal comes from to_string, not parenthesize, so its capacity is not set exactly
    let printer = AstPrinter::new(&num(7.0)).unwrap();
    assert_eq!(printer.text, "7");
    assert!(printer.text.capacity() >= printer.text.len());
}

// ---------------- Display ----------------

#[test]
fn display_shows_the_stored_text() {
    let printer = AstPrinter::new(&group(num(1.0))).unwrap();
    assert_eq!(format!("{printer}"), printer.text);
}

#[test]
fn display_respects_width() {
    // Display uses pad, so width and alignment work
    let printer = AstPrinter::new(&group(num(1.0))).unwrap();
    assert_eq!(format!("{printer:>12}"), "   (group 1)");
    assert_eq!(format!("{printer:<12}|"), "(group 1)   |");
}

#[test]
fn display_can_be_called_many_times() {
    let printer = AstPrinter::new(&num(1.0)).unwrap();
    assert_eq!(printer.to_string(), printer.to_string());
}

// ---------------- example main ----------------

#[test]
fn example_main_runs() {
    main();
}
