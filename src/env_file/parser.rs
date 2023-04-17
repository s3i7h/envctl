use crate::env_file::{EnvDeclaration, EnvFile, EnvFileRow};
use combine::parser::char::{char, newline, space};
use combine::{many, many1, optional, satisfy, sep_end_by, skip_many, ParseError, Parser, Stream};
use combine::parser::choice::or;

fn name<Input>() -> impl Parser<Input, Output = String>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (
        satisfy(|c: char| c == '_' || c.is_alphabetic())
            .and(many::<String, _, _>(satisfy(|c: char| c == '_' || c.is_alphanumeric()))),
        char('='),
    )
        .map(|((head, rest), _)| format!("{}{}", head, rest))
        .message("while parsing env_declaration")
}

fn value<Input>() -> impl Parser<Input, Output = String>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    many1(satisfy(|c: char| !c.is_whitespace())).message("while parsing value")
}

fn declaration<Input>() -> impl Parser<Input, Output = EnvFileRow>
    where
        Input: Stream<Token = char>,
        Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (name(), skip_many(satisfy(|c: char| c.is_whitespace() && c != '\n')), optional(value()))
        .map(|(name, _, value)| EnvFileRow::Declaration(EnvDeclaration { name, value }))
        .message("while parsing value")
}

fn comment<Input>() -> impl Parser<Input, Output = EnvFileRow>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (char('#'), skip_many(space()), many1(satisfy(|c| c != '\n')))
        .map(|(_, _, comment)| EnvFileRow::CommentOnly(comment))
        .message("while parsing comment")
}

fn row<Input>() -> impl Parser<Input, Output = EnvFileRow>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    optional(or(
        declaration(),
        comment(),
    ))
        .map(|result| match result {
            None => EnvFileRow::Empty,
            Some(row) => row,
        })
        .message("while parsing row")
}

pub(super) fn env_file<Input>() -> impl Parser<Input, Output = EnvFile>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    sep_end_by(row(), newline())
        .map(EnvFile)
        .message("while parsing env_file")
}

#[test]
fn test_name_parser() {
    let input = "ABC=";
    let (result, next) = name().parse(input).expect("parse failed");
    println!("{}", next);
    println!("{:?}", result);
}

#[test]
fn test_row_parser() {
    let input = "\nabc=";
    let (result, next) = row().parse(input).expect("parse failed");
    println!("{}", next);
    println!("{:?}", result);
}

#[test]
fn test_env_file_parser() {
    let input = r#"# comment only

ABC=
WITH_DEFAULT=123
WITH_COMMENT=
WITH_NUMBER_IN_0=
"#;
    let (result, next) = env_file().parse(input).expect("parse failed");
    assert_eq!(next, "");
    assert_eq!(
        result,
        EnvFile(vec![
            EnvFileRow::CommentOnly("comment only".to_string()),
            EnvFileRow::Empty,
            EnvFileRow::Declaration(EnvDeclaration {
                name: "ABC".to_string(),
                value: None,
            }),
            EnvFileRow::Declaration(EnvDeclaration {
                name: "WITH_DEFAULT".to_string(),
                value: Some("123".to_string()),
            }),
            EnvFileRow::Declaration(EnvDeclaration {
                name: "WITH_COMMENT".to_string(),
                value: None,
            }),
            EnvFileRow::Declaration(EnvDeclaration {
                name: "WITH_NUMBER_IN_0".to_string(),
                value: None,
            }),
            EnvFileRow::Empty
        ])
    )
}
