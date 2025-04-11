use logos::{Logos, Lexer};
use lexical_core;

/// Lua language tokens.
///
/// No support for nested comments or nested multiline strings.
///
/// `--[==[This is a nested comment--]==]`
#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
#[logos(skip r"--[^\n]*")]
#[logos(skip r"--\[\[(.|\n)--\]\]")]
#[logos(skip r"\#\![^\n]*")]
pub enum LuaToken<'source> {
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
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    //==----------
    // Identifier
    //==----------
    #[regex("[a-zA-Z_][a-zA-Z_0-9]*", |text| text.slice())]
    Identifier(&'source str),
    /*
    #[regex(r"[a-zA-Z_][a-zA-Z_0-9]*\[[a-zA-Z_][a-zA-Z_]*\]")]
    IdTableIndex((&'source str, &'source str)),
    #[regex(r#"\"[a-zA-Z_][a-zA-Z_0-9]*\[[a-zA-Z_][a-zA-Z_]*\]\""#)]
    StringTableIndex((&'source str, &'source str)),
    */
    //==---------------
    // String literals
    //==---------------
    #[regex("\"[^\"\n]*\"|'[^'\n]*'", |text| text.slice()[1..text.slice().len()-1].trim())]
    String(&'source str),
    /*
    #[regex("'([^'\n]*)'", |text| text.slice())]
    SingleQuoteString(&'source str),
    */
    /*
    #[regex(r"\[\[(.|\n)\]\]")]
    MultipleLineString(&'source str),
    */
    //==---------------
    // Number literals
    //==---------------
    #[regex(r"[0-9][0-9_]*|0[xX][0-9a-fA-F][0-9a-fA-F_]*", as_int)]
    Integer(i64),
    #[regex(r"([0-9][0-9_]*\.[0-9][0-9_]*)|(0[xX][0-9a-fA-F][0-9a-fA-F_]*\.[0-9a-fA-F][0-9a-fA-F_]*)", as_float)]
    Float(f64),
}

fn as_int<'source>(text: &mut Lexer<'source, LuaToken<'source>>) -> Option<i64> {
    let s: String = text.slice().trim_start_matches("0x").trim_start_matches("0X").replace("_","");

    // Add scientific notation handling (e & p).

    let as_int: Result<i64, lexical_core::Error> = lexical_core::parse(s.as_bytes());

    match as_int {
        Ok(val) => Some(val),
        Err(_) => None,
    }

}

fn as_float<'source>(text: &mut Lexer<'source, LuaToken<'source>>) -> Option<f64> {

    let s= text.slice().trim_start_matches("0x").trim_start_matches("0X").replace("_","");

    // Add scientific notation handling (e & p).

    let as_float: Result<f64, lexical_core::Error> = lexical_core::parse(s.as_bytes());

    match as_float {
        Ok(val) => Some(val),
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_single_quote_str() {
        let mut lex = LuaToken::lexer("'This is a single line, single quoted string'");
        assert_eq!(lex.next(), Some(Ok(LuaToken::String("This is a single line, single quoted string"))));
        assert_eq!(lex.slice(), "'This is a single line, single quoted string'");
    }
    #[test]
    fn lex_double_quote_str() {
        let mut lex = LuaToken::lexer("\"This is a single line, double quoted string\"");
        assert_eq!(lex.next(), Some(Ok(LuaToken::String("This is a single line, double quoted string"))));
        assert_eq!(lex.slice(), "\"This is a single line, double quoted string\"");
    }
    #[test]
    fn lex_id() {
        let mut lex = LuaToken::lexer("id1 Id2 Id_3");
        assert_eq!(lex.next(), Some(Ok(LuaToken::Identifier("id1"))));
        assert_eq!(lex.next(), Some(Ok(LuaToken::Identifier("Id2"))));
        assert_eq!(lex.next(), Some(Ok(LuaToken::Identifier("Id_3"))));
    }
    #[test]
    fn lex_int() {
        let mut lex = LuaToken::lexer("0 1 0x1 9 10 99 1_000");
        assert_eq!(lex.next(), Some(Ok(LuaToken::Integer(0))));
        assert_eq!(lex.next(), Some(Ok(LuaToken::Integer(1))));
        assert_eq!(lex.next(), Some(Ok(LuaToken::Integer(1))));
        assert_eq!(lex.next(), Some(Ok(LuaToken::Integer(9))));
        assert_eq!(lex.next(), Some(Ok(LuaToken::Integer(10))));
        assert_eq!(lex.next(), Some(Ok(LuaToken::Integer(99))));
        assert_eq!(lex.next(), Some(Ok(LuaToken::Integer(1_000))));
    }
    #[test]
    fn lex_float() {
        let mut lex = LuaToken::lexer("0.0 1.0 0x1.1 9.0 10.123_4 0_.99 1_000.0000_000");
        assert_eq!(lex.next(), Some(Ok(LuaToken::Float(0.0))));
        assert_eq!(lex.next(), Some(Ok(LuaToken::Float(1.0))));
        assert_eq!(lex.next(), Some(Ok(LuaToken::Float(1.1))));
        assert_eq!(lex.next(), Some(Ok(LuaToken::Float(9.0))));
        assert_eq!(lex.next(), Some(Ok(LuaToken::Float(10.1234))));
        assert_eq!(lex.next(), Some(Ok(LuaToken::Float(0.99))));
        assert_eq!(lex.next(), Some(Ok(LuaToken::Float(1000.00000000))));
    }
    #[test]
    fn lex_array_and_table() {
        let mut lex = LuaToken::lexer("my_array[1] other_array[\"x\"]");
        assert_eq!(lex.next(), Some(Ok(LuaToken::Identifier("my_array"))));
        assert_eq!(lex.next(), Some(Ok(LuaToken::LBracket)));
        assert_eq!(lex.next(), Some(Ok(LuaToken::Integer(1))));
        assert_eq!(lex.next(), Some(Ok(LuaToken::RBracket)));

        assert_eq!(lex.next(), Some(Ok(LuaToken::Identifier("other_array"))));
        assert_eq!(lex.next(), Some(Ok(LuaToken::LBracket)));
        assert_eq!(lex.next(), Some(Ok(LuaToken::String("x"))));
        assert_eq!(lex.next(), Some(Ok(LuaToken::RBracket)));
    }
    /*
    #[test]
    fn lex_table_index() {
        let mut lex = LuaToken::lexer("args[1] other_args[1_0]");
        assert_eq!(lex.next(), Some(Ok(LuaToken::TableIndex(("args",1)))));
        assert_eq!(lex.next(), Some(Ok(LuaToken::TableIndex(("other_args",10)))));
    }
    */
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


