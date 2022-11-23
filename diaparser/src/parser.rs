use nom::{
    combinator::{
        eof
    }, InputTake, sequence::pair
};
use nom_supreme::{error::{GenericErrorTree}};

use crate::{token::{TokSpan, Tokens, Spanned}, recovery_err::{ToRange, RecoveredError, TokError, TokResult}, Expr};

mod statements;

use statements::statements;

fn parser<'a, 'b>(input: Tokens<'a, 'b>, empty_vec: &'a [TokSpan<'a, 'b>]) -> Spanned<Expr> {
    pair(
        |input: Tokens<'a, 'b>| -> TokResult<'a, 'b, Spanned<Expr>> {match statements(input) {
            Ok(r) => Ok(r),
            Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => {
                let input = recover_err(&e);
                let (rest, input) = input.take_split(1);
                let span = input.span();
                let state = input.tokens[0].extra; 
                state.0.report_error(RecoveredError(span.clone(), "Unexpected token".to_string()));
                Ok((rest, (Expr::Error, span)))
            },
            Err(e) => Err(e)
        }},
        eof
    )(input).expect("Unrecovered error happened in parser").1.0
}

fn recover_err<'a, 'b>(e: &TokError<'a, 'b>) -> Tokens<'a, 'b> {
    match e {
        GenericErrorTree::Base { location, kind: _ } => *location,
        GenericErrorTree::Stack { base, contexts: _ } => recover_err(base),
        GenericErrorTree::Alt(v) => recover_err(v.get(0).unwrap()),
    }
}

// Hashmap contains the names of named expressions and their clones
pub fn token_parse(tokens: Vec<TokSpan>) -> Spanned<Expr> {
    let empty_vec = vec![];
    let ex = match tokens.len() {
        0 => (Expr::Error, 0..1),
        _ => parser(Tokens::new(&tokens, tokens[0].extra.0), &empty_vec)
    };
    //let ex = (Expr::Dollar, 0..1);
    ex
}
