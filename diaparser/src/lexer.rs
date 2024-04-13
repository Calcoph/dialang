use std::cell::RefCell;

use nom::{
    character::complete::{
        alpha1,
        alphanumeric1,
        multispace1,
        not_line_ending
    },
    branch::alt as choice,
    bytes::complete::{
        tag, take_until
    },
    combinator::{
        recognize,
        eof,
        map,
        map_opt
    },
    sequence::{pair, preceded, delimited},
    multi::{
        many0, many_till
    },
    InputTake
};
use nom_supreme::error::{GenericErrorTree, ErrorTree};

use crate::{recovery_err::{ParseState, RecoveredError, StrResult, StrSpan, ToRange}, token::{FromStrSpan, Keyword, TokSpan, Token}, Annotation};

fn lexer<'a, 'b>(input: StrSpan<'a, 'b>) -> StrResult<StrSpan<'a, 'b>, Vec<TokSpan<'a, 'b>>> {
    // A parser for operators
    let op = map(
        tag("="),
        |s: StrSpan| {
            let state = s.extra;
            TokSpan::from_strspan(Token::Op(s.fragment()), state, s.span())
        }
    );

    // A parser for control characters (delimiters, semicolons, etc.)
    let ctrl = map(
        choice((
            tag("("),
            tag(")"),
            tag("{"),
            tag("}"),
            tag(","),
            tag("."),
            tag(":"),
            tag("#")
        )),
        |s: StrSpan| {
            let state = s.extra;
            TokSpan::from_strspan(
                Token::Separator(s.fragment().chars().next().unwrap()),
                state,
                s.span()
            )
        }
    );

    let annotation = map_opt(
        preceded(
            tag("@"),
            tag("SequenceEntrypoint"),
        ),
        |s: StrSpan| {
            let token = match *s.fragment() {
                "SequenceEntrypoint" => Some(Token::A(Annotation::SequenceEntrypoint)),
                _ => None
            };
            let state = s.extra;
            token.map(|token| TokSpan::from_strspan(token, state, s.span()))
        }
    );

    // A parser for identifiers and keywords
    let ident = map(
        recognize(
            pair(
                choice((alpha1, tag("_"))),
                many0(choice((alphanumeric1, tag("_"))))
            )
        ),
        |s: StrSpan| {
            let token = match *s.fragment() {
                "fn" => Token::K(Keyword::Fn),
                "if" => Token::K(Keyword::If),
                "else" => Token::K(Keyword::Else),
                "while" => Token::K(Keyword::While),
                "for" => Token::K(Keyword::For),
                "in" => Token::K(Keyword::In),
                "class" => Token::K(Keyword::Class),
                "struct" => Token::K(Keyword::Class),
                s => Token::Ident(s)
            };
            let state = s.extra;
            TokSpan::from_strspan(token, state, s.span())
        }
    );

    let raw = map(
        delimited(
            tag("`"),
            recognize(take_until("`")),
            tag("`")
        ),
        |s: StrSpan| {
            TokSpan::from_strspan(Token::Ident(s.fragment()), s.extra, s.span())
        }
    );

    // A single token can be one of the above
    let token = choice((
        raw,
        op,
        ctrl,
        annotation,
        ident,
    ));

    let comment = preceded(tag("//"), not_line_ending);

    let padding = map(
        choice((
            comment,
            multispace1
        )),
        |s: StrSpan| {
            let state = s.extra;
            TokSpan::from_strspan(Token::Comment(s.fragment()), state, s.span())
        }
    );

    let mut pos_inputs = choice((padding, token));

    map(
        many_till(
            move |input: StrSpan<'a, 'b>| match pos_inputs(input) {
                Ok(r) => Ok(r),
                Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => {
                    let input = recover_err(&e);
                    let len = input.fragment().chars().next().unwrap().len_utf8();
                    let (rest, input) = input.take_split(len);
                    let span = input.span();
                    input.extra.report_error(RecoveredError(span.clone(), "Unkown (non-ASCII) character".to_string()));
                    let state = rest.extra;
                    Ok((rest, TokSpan::from_strspan(Token::Err, state, span)))
                },
                Err(e) => Err(e)
            },
            eof
        ),
        |(v, _)| v.into_iter().filter(|a| a.not_comment()).collect()
    )(input)
}

fn recover_err<'a, 'b>(e: &ErrorTree<StrSpan<'a, 'b>>) -> StrSpan<'a, 'b> {
    match e {
        GenericErrorTree::Base { location, kind: _ } => *location,
        GenericErrorTree::Stack { base, contexts: _ } => recover_err(base),
        GenericErrorTree::Alt(v) => recover_err(v.get(0).unwrap()),
    }
}

pub fn lex<'a, 'b>(input: &'a str, errors: &'b RefCell<Vec<RecoveredError>>) -> Vec<TokSpan<'a, 'b>> {
    let input = StrSpan::new_extra(input, ParseState(errors));
    let (_, tokens) = lexer(input).expect("Unrecovered error happenned in lexer");

    tokens
}

pub fn lex_tokens<'a, 'b>(input: &'a str, errors: &'b RefCell<Vec<RecoveredError>>) -> Vec<Token<'a>> {
    let input = StrSpan::new_extra(input, ParseState(errors));
    let (_, tokens) = lexer(input).expect("Unrecovered error happenned in lexer");

    tokens.into_iter().map(|a| *a.fragment()).collect()
}
