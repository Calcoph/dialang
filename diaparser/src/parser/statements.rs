use std::{ops::Range, io::ErrorKind};

use nom::{branch::alt as choice, bytes::complete::{tag, take}, combinator::{map, map_res, opt}, multi::{many0, separated_list0}, sequence::{delimited, pair, preceded, terminated, tuple}};
use nom_supreme::{error::{ErrorTree, BaseErrorKind}, ParserExt};

use crate::{combinators::{map_with_span, spanned}, recovery_err::{expression_recovery, non_opt, TokResult}, token::{Keyword, Spanned, Token, Tokens}, AnnotatedBlock, Annotation, Assignment, Attribute, Class, Expr, FuncCall, Method, ParserError, SequenceEntrypointBlock, TopLevelStatement};

pub(crate) fn ident<'a, 'b>(input: Tokens<'a, 'b>) -> TokResult<'a, 'b, Spanned<String>> {
    map_res(
        spanned(take(1 as usize)),
        |(consumed, span): (Tokens, Range<usize>)| {
            match consumed.tokens[0].fragment() {
                Token::Ident(s) => Ok((String::from(*s), span)),
                _ => Err(ErrorTree::Base {
                    location: consumed,
                    kind: BaseErrorKind::External(Box::new(tokio::io::Error::new(ErrorKind::Other, "Expected identifier")))
                }) // TODO: Expand match tree for "expected: _, found X"
            }
        }
    )(input)
}

fn attribute<'a, 'b>(input: Tokens<'a, 'b>) -> TokResult<'a, 'b, Spanned<Attribute>> {
    map_with_span(
        pair(
            ident,
            opt(preceded(
                tag(Token::Separator(':')),
                non_opt(ident)
            )),
        ),
        |(name, r#type), span| (Attribute {
            name,
            r#type
        }, span)
    )(input)
}

fn method_definition<'a, 'b>(input: Tokens<'a, 'b>) -> TokResult<'a, 'b, Spanned<Method>> {
    map_with_span(
        preceded(
            tag(Token::K(Keyword::Fn)),
            tuple((
                ident,
                delimited(
                    tag(Token::Separator('(')),
                    separated_list0(tag(Token::Separator(',')), attribute),
                    tag(Token::Separator(')'))
                ),
                opt(preceded(
                    tag(Token::Separator(':')),
                    non_opt(ident)
                )),
                opt(preceded(
                    tag(Token::Separator('{')),
                    non_opt(terminated(
                        method_body,
                        tag(Token::Separator('}'))
                    ))
                ))
            ))
        ),
        |(name, parameters, ret_type, body), span| (
            Method {
                name,
                parameters,
                ret_type,
                body
            }, span)
    )(input)
}

fn func_call<'a, 'b>(input: Tokens<'a, 'b>) -> TokResult<'a, 'b, Spanned<FuncCall>> {
    map_with_span(
        tuple((
            ident,
            opt(preceded(
                tag(Token::Separator('.')),
                non_opt(ident)
            )),
            delimited(
                tag(Token::Separator('(')),
                separated_list0(tag(Token::Separator(',')), ident),
                tag(Token::Separator(')'))
            )
        )),
        |(root, access, args), span| (FuncCall {
            root,
            access,
            args
        }, span)
    )(input)
}

fn method_body<'a, 'b>(input: Tokens<'a, 'b>) -> TokResult<'a, 'b, Spanned<Expr>> {
    map_with_span(
        many0(map_with_span(
            pair(
                opt(terminated(
                    pair(
                        ident,
                        opt(preceded(
                            tag(Token::Separator(':')),
                            ident
                        )),
                    ),
                    tag(Token::Op("="))
                )),
                func_call
            ),
            |(assignment, (f_call, f_call_span)), span| {
                match assignment {
                    Some((assignment, r#type)) => (Expr::Assignment(Box::new(Assignment {
                        name: assignment,
                        expr: (Expr::FuncCall(Box::new(f_call)), f_call_span),
                        r#type: r#type,
                    })), span),
                    None => (Expr::FuncCall(Box::new(f_call)), span)
                }
            }
        )),
        |funs, span| (Expr::ExprList(funs), span)
    )(input)
}

fn class_definition<'a, 'b>(input: Tokens<'a, 'b>) -> TokResult<'a, 'b, Spanned<Class>> {
    map_with_span(
        pair(
            preceded(
                tag(Token::K(Keyword::Class)).context("tag class"),
                ident.context("class name")
            ),
            delimited(
                tag(Token::Separator('{')).context("Opening brack"),
                pair(
                    many0(attribute),
                    many0(method_definition)
                ),
                tag(Token::Separator('}')).context("Closing brack")
            )
        ),
        |(name, (attributes, methods)), span| (Class {
            name,
            attributes,
            methods
        }, span)
    )(input)
}

fn annotation_entrypoint<'a, 'b>(input: Tokens<'a, 'b>) -> TokResult<'a, 'b, Spanned<SequenceEntrypointBlock>> {
    map_with_span(
        preceded(
            tag(Token::A(Annotation::SequenceEntrypoint)).context("tag sequence_entrypoint"),
            func_call
        ),
        |func_call, span| (SequenceEntrypointBlock {
            function: func_call
        }, span)
    )(input)
}

fn annotation_block<'a, 'b>(input: Tokens<'a, 'b>) -> TokResult<'a, 'b, Spanned<AnnotatedBlock>> {
    choice((
        map(annotation_entrypoint, |(entry_point_block, span)| (AnnotatedBlock::SequenceEntrypoint(entry_point_block), span)),
    ))(input)
}

pub(crate) fn statements<'a, 'b>(input: Tokens<'a, 'b>) -> TokResult<'a, 'b, Spanned<Result<Vec<Spanned<TopLevelStatement>>, ParserError>>> {
    expression_recovery(map_with_span(
        many0(choice((
            map(class_definition, |(class, s)| (TopLevelStatement::Class(class), s)),
            map(annotation_block, |(annotated, s)| (TopLevelStatement::AnnotatedBlock(annotated), s)),
        ))),
        |v, span| (
                    Ok(v),
                    span
                )
    ))(input)
}
