use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")] // Ignore this regex pattern between tokens
enum Token {
    // Tokens can be literal strings, of any length.
    #[token("fast")]
    Fast,

    #[token(".")]
    Period,

    // Or regular expressions.
    #[regex("[a-zA-Z]+")]
    Text,
}

fn main() {
    let mut lex = Token::lexer("Create ridiculously fast Lexers.");

    assert_eq!(lex.next(), Some(Ok(Token::Text)));
    assert_eq!(lex.span(), 0..6);
    assert_eq!(lex.slice(), "Create");

    assert_eq!(lex.next(), Some(Ok(Token::Text)));
    assert_eq!(lex.span(), 7..19);
    assert_eq!(lex.slice(), "ridiculously");

    assert_eq!(lex.next(), Some(Ok(Token::Fast)));
    assert_eq!(lex.span(), 20..24);
    assert_eq!(lex.slice(), "fast");

    assert_eq!(lex.next(), Some(Ok(Token::Text)));
    assert_eq!(lex.slice(), "Lexers");
    assert_eq!(lex.span(), 25..31);

    assert_eq!(lex.next(), Some(Ok(Token::Period)));
    assert_eq!(lex.span(), 31..32);
    assert_eq!(lex.slice(), ".");

    assert_eq!(lex.next(), None);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_readme_example() {
        #[derive(logos::Logos, Debug, PartialEq)]
        #[logos(skip r"[ \t\n\f]+")]
        enum Token {
            #[token("fast")]
            Fast,

            #[token(".")]
            Period,

            #[regex("[a-zA-Z]+")]
            Text,
        }

        let mut lexer = <Token as logos::Logos>::lexer("Create ridiculously fast Lexers.");

        assert_eq!(lexer.next(), Some(Ok(Token::Text)));
        assert_eq!(lexer.span(), 0..6);
        assert_eq!(lexer.slice(), "Create");

        assert_eq!(lexer.next(), Some(Ok(Token::Text)));
        assert_eq!(lexer.span(), 7..19);
        assert_eq!(lexer.slice(), "ridiculously");

        assert_eq!(lexer.next(), Some(Ok(Token::Fast)));
        assert_eq!(lexer.span(), 20..24);
        assert_eq!(lexer.slice(), "fast");

        assert_eq!(lexer.next(), Some(Ok(Token::Text)));
        assert_eq!(lexer.slice(), "Lexers");
        assert_eq!(lexer.span(), 25..31);

        assert_eq!(lexer.next(), Some(Ok(Token::Period)));
        assert_eq!(lexer.span(), 31..32);
        assert_eq!(lexer.slice(), ".");

        assert_eq!(lexer.next(), None);
    }

    #[derive(Debug, PartialEq, logos::Logos)]
    #[logos(skip r" ")]
    enum Token2 {
        #[regex("[0-9]+")]
        Int,
        #[token("+")]
        Plus,
    }

    #[test]
    fn test() {
        let mut lexer = <Token2 as logos::Logos>::lexer("1 + 2");

        assert_eq!(lexer.next(), Some(Ok(Token2::Int)));
        assert_eq!(lexer.span(), 0..1);
        assert_eq!(lexer.slice(), "1");

        assert_eq!(lexer.next(), Some(Ok(Token2::Plus)));
        assert_eq!(lexer.span(), 2..3);
        assert_eq!(lexer.slice(), "+");

        assert_eq!(lexer.next(), Some(Ok(Token2::Int)));
        assert_eq!(lexer.span(), 4..5);
        assert_eq!(lexer.slice(), "2");

        assert_eq!(lexer.next(), None);
    }

    #[derive(Debug, PartialEq, logos::Logos)]
    enum Token3 {
        #[regex("[1-9][0-9]{3}-[0-9]{2}-[0-9]{2}", |lexer| {
            let slice = lexer.slice();
            let year = slice[0..4].parse::<u16>().expect("year");
            let month = slice[5..7].parse::<u8>().expect("month");
            let day_of_month = slice[8..10].parse::<u8>().expect("day_of_month");
            (year, month, day_of_month)
        })]
        Date((u16, u8, u8)),
    }

    #[test]
    fn test_token3() {
        let mut lexer = <Token3 as logos::Logos>::lexer("2020-01-02");

        assert_eq!(lexer.next(), Some(Ok(Token3::Date((2020_u16, 1_u8, 2_u8)))));
        assert_eq!(lexer.span(), 0..10);
        assert_eq!(lexer.slice(), "2020-01-02");

        assert_eq!(lexer.next(), None);
    }

    #[derive(Debug, PartialEq, logos::Logos)]
    #[logos(skip r"\s+")]
    enum Token4 {
        #[token("false")]
        False,
        #[token("true")]
        True,
        #[token("?")]
        Question,
        #[token(":")]
        Colon,
        #[regex("0|[1-9][0-9]{0,}", |lexer| lexer.slice().parse::<u8>().ok())]
        Integer(u8),
        #[token("+")]
        Plus,
    }

    #[test]
    fn test_token4() {
        let mut lexer = <Token4 as logos::Logos>::lexer("true ? 0 : 1 + 2");

        assert_eq!(lexer.next(), Some(Ok(Token4::True)));
        assert_eq!(lexer.span(), 0..4);
        assert_eq!(lexer.slice(), "true");

        assert_eq!(lexer.next(), Some(Ok(Token4::Question)));
        assert_eq!(lexer.span(), 5..6);
        assert_eq!(lexer.slice(), "?");

        assert_eq!(lexer.next(), Some(Ok(Token4::Integer(0))));
        assert_eq!(lexer.span(), 7..8);
        assert_eq!(lexer.slice(), "0");

        assert_eq!(lexer.next(), Some(Ok(Token4::Colon)));
        assert_eq!(lexer.span(), 9..10);
        assert_eq!(lexer.slice(), ":");

        assert_eq!(lexer.next(), Some(Ok(Token4::Integer(1))));
        assert_eq!(lexer.span(), 11..12);
        assert_eq!(lexer.slice(), "1");

        assert_eq!(lexer.next(), Some(Ok(Token4::Plus)));
        assert_eq!(lexer.span(), 13..14);
        assert_eq!(lexer.slice(), "+");

        assert_eq!(lexer.next(), Some(Ok(Token4::Integer(2))));
        assert_eq!(lexer.span(), 15..16);
        assert_eq!(lexer.slice(), "2");

        assert_eq!(lexer.next(), None);
    }

    // unary = false | true | integer
    // binary = unary | unary plus binary
    // ternary = binary | binary question ternary colon ternary

    #[derive(Debug, PartialEq)]
    enum Term {
        False,
        True,
        Integer(u8),
        Add(Box<Term>, Box<Term>),
        If(Box<Term>, Box<Term>, Box<Term>),
    }

    fn read_binary(iter: &mut std::iter::Peekable<logos::Lexer<'_, Token4>>) -> Term {
        let token = iter.next().unwrap().unwrap();
        let unary = match token {
            Token4::False => Term::False,
            Token4::True => Term::True,
            Token4::Integer(i) => Term::Integer(i),
            Token4::Plus | Token4::Question | Token4::Colon => unreachable!(),
        };
        match iter.peek() {
            None => unary,
            Some(token) => {
                let token = token.as_ref().unwrap();
                match token {
                    Token4::False | Token4::True | Token4::Integer(_) => unreachable!(),
                    Token4::Question | Token4::Colon => unary,
                    Token4::Plus => {
                        assert!(matches!(iter.next(), Some(Ok(Token4::Plus))));
                        let left = unary;
                        let right = read_binary(iter);
                        Term::Add(Box::new(left), Box::new(right))
                    }
                }
            }
        }
    }

    fn read_ternary(iter: &mut std::iter::Peekable<logos::Lexer<'_, Token4>>) -> Term {
        let binary = read_binary(iter);
        match iter.peek() {
            None => binary,
            Some(token) => {
                let token = token.as_ref().unwrap();
                match token {
                    Token4::False | Token4::True | Token4::Integer(_) | Token4::Plus => {
                        unreachable!()
                    }
                    Token4::Question => {
                        assert!(matches!(iter.next(), Some(Ok(Token4::Question))));
                        let cond = binary;
                        let thn = read_ternary(iter);
                        assert!(matches!(iter.next(), Some(Ok(Token4::Colon))));
                        let els = read_ternary(iter);
                        Term::If(Box::new(cond), Box::new(thn), Box::new(els))
                    }
                    Token4::Colon => binary,
                }
            }
        }
    }

    #[test]
    fn test_token4_with_expr() {
        fn test_parse(s: &str, expected: Term) {
            let lexer = <Token4 as logos::Logos>::lexer(s);
            let term = read_ternary(&mut lexer.peekable());
            assert_eq!(term, expected);
        }

        test_parse("true", Term::True);
        test_parse("false", Term::False);
        test_parse("0", Term::Integer(0));
        test_parse(
            "1 + 2",
            Term::Add(Box::new(Term::Integer(1)), Box::new(Term::Integer(2))),
        );
        test_parse(
            "3 + 4 + 5",
            Term::Add(
                Box::new(Term::Integer(3)),
                Box::new(Term::Add(
                    Box::new(Term::Integer(4)),
                    Box::new(Term::Integer(5)),
                )),
            ),
        );
        test_parse(
            "true ? 6 : 7",
            Term::If(
                Box::new(Term::True),
                Box::new(Term::Integer(6)),
                Box::new(Term::Integer(7)),
            ),
        );
        test_parse(
            "true ? true ? 8 : 9 : 10",
            Term::If(
                Box::new(Term::True),
                Box::new(Term::If(
                    Box::new(Term::True),
                    Box::new(Term::Integer(8)),
                    Box::new(Term::Integer(9)),
                )),
                Box::new(Term::Integer(10)),
            ),
        );
        test_parse(
            "true ? 11 : true ? 12 : 13",
            Term::If(
                Box::new(Term::True),
                Box::new(Term::Integer(11)),
                Box::new(Term::If(
                    Box::new(Term::True),
                    Box::new(Term::Integer(12)),
                    Box::new(Term::Integer(13)),
                )),
            ),
        );
    }

    #[test]
    fn test_typecheck() {
        #[derive(Debug, PartialEq)]
        enum Type {
            Boolean,
            Integer,
        }

        fn typecheck(t: Term) -> Type {
            match t {
                Term::False | Term::True => Type::Boolean,
                Term::Integer(_) => Type::Integer,
                Term::Add(left, right) => {
                    assert_eq!(typecheck(*left), Type::Integer, "integer expected");
                    assert_eq!(typecheck(*right), Type::Integer, "integer expected");
                    Type::Integer
                }
                Term::If(cond, thn, els) => {
                    assert_eq!(typecheck(*cond), Type::Boolean, "boolean expected");
                    let thn_type = typecheck(*thn);
                    assert_eq!(
                        thn_type,
                        typecheck(*els),
                        "then and else have different types"
                    );
                    thn_type
                }
            }
        }

        let lexer = <Token4 as logos::Logos>::lexer("false");
        let term = read_ternary(&mut lexer.peekable());
        assert_eq!(typecheck(term), Type::Boolean);

        let lexer = <Token4 as logos::Logos>::lexer("true");
        let term = read_ternary(&mut lexer.peekable());
        assert_eq!(typecheck(term), Type::Boolean);

        let lexer = <Token4 as logos::Logos>::lexer("0");
        let term = read_ternary(&mut lexer.peekable());
        assert_eq!(typecheck(term), Type::Integer);

        let lexer = <Token4 as logos::Logos>::lexer("true ? 0 : 1 + 2");
        let term = read_ternary(&mut lexer.peekable());
        assert_eq!(typecheck(term), Type::Integer);
    }
}
