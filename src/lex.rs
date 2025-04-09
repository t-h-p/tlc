use logos::Logos;


/// Lua language tokens.
///
/// No support for nested comments or nested multiline strings.
///
/// `--[==[This is a nested comment--]==]`
#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
#[logos(skip r"--[^\n]*")]
#[logos(skip r"--\[\[(.|\n)--\]\]")]
enum LuaToken<'source> {
    //==--------
    // Keywords
    //==--------
    #[token("and")]
    And,
    #[token("break")]
    Break,
    #[token("continue")]
    Continue,
    #[token("do")]
    Do,
    #[token("else")]
    Else,
    #[token("elseif")]
    Elseif,
    #[token("end")]
    End,
    #[token("false")]
    False,
    #[token("for")]
    For,
    #[token("function")]
    Function,
    #[token("goto")]
    Goto,
    #[token("if")]
    If,
    #[token("in")]
    In,
    #[token("local")]
    Local,
    #[token("nil")]
    Nil,
    #[token("not")]
    Not,
    #[token("or")]
    Or,
    #[token("repeat")]
    Repeat,
    #[token("return")]
    Return,
    #[token("then")]
    Then,
    #[token("true")]
    True,
    #[token("until")]
    Until,
    #[token("while")]
    While, 
    //==---------
    // Operators
    //==---------
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Multiply,
    #[token("/")]
    Divide,
    #[token("%")]
    Modulus,
    #[token("^")]
    Exponent,
    #[token("==")]
    DoubleEqual,
    #[token("=")]
    Equal,
    #[token("~=")]
    NotEqual,
    #[token(">")]
    Greater,
    #[token("<")]
    Less,
    #[token(">=")]
    GreaterEqual,
    #[token("<=")]
    LessEqual,
    #[token("..")]
    Concatenate,
    #[token("#")]
    Length,
    //==----------
    // Identifier
    //==----------
    #[regex("[a-zA-Z_][a-zA-Z_0-9]*", |text| text.slice())]
    Identifier(&'source str),
    //==---------------
    // String literals
    //==---------------
    #[regex("\"([^\"\n]*)\"", |text| text.slice())]
    DoubleQuoteString(&'source str),
    #[regex("'([^'\n]*)'", |text| text.slice())]
    SingleQuoteString(&'source str),
    /*
    #[regex(r"\[\[(.|\n)\]\]")]
    MultipleLineString(&'source str),
    */
    //==---------------
    // Number literals
    //==---------------
    #[regex(r"[0-9][0-9_]*", |text| text.slice())]
    Integer(&'source str),
    #[regex("0x[0-9a-fA-F]")]
    HexInteger(&'source str),
    #[regex("[0-9]*[0-9]*")]
    Float(&'source str),

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_single_quote_str() {
        let mut lex = LuaToken::lexer("'This is a single line, single quoted string'");
        assert_eq!(lex.next(), Some(Ok(LuaToken::SingleQuoteString("'This is a single line, single quoted string'"))));
        assert_eq!(lex.slice(), "'This is a single line, single quoted string'");
    }
    #[test]
    fn lex_double_quote_str() {
        let mut lex = LuaToken::lexer("\"This is a single line, double quoted string\"");
        assert_eq!(lex.next(), Some(Ok(LuaToken::DoubleQuoteString("\"This is a single line, double quoted string\""))));
        assert_eq!(lex.slice(), "\"This is a single line, double quoted string\"");
    }
    #[test]
    fn lex_id() {
        let mut lex = LuaToken::lexer("id1 Id2 Id_3");
        assert_eq!(lex.next(), Some(Ok(LuaToken::Identifier("id1"))));
        assert_eq!(lex.next(), Some(Ok(LuaToken::Identifier("Id2"))));
        assert_eq!(lex.next(), Some(Ok(LuaToken::Identifier("Id_3"))));
    }
    /*
    #[test]
    fn lex_multiple_line_str() {
        let mut lex = LuaToken::lexer("[[This is a multiple line,\n string :) ]]");
        lex.next();
        println!("Token: {}",lex.slice());
        assert_eq!(lex.next(), Some(Ok(LuaToken::MultipleLineString("[[This is a multiple line,\n string :) ]]"))));
        assert_eq!(lex.slice(), "[[This is a multiple line,\n string :) ]]");
    }
    */ // No support for multiline strings currently

}


