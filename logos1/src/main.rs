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
}
