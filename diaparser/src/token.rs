use std::{ops::Range, iter::{Enumerate, Copied}, slice::Iter, fmt::{self, Display, Debug}};

use nom::{Compare, CompareResult, InputLength, InputIter, InputTake, Needed};
use nom_locate::LocatedSpan;

use crate::{recovery_err::{ParseState, ToRange}, Annotation};

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Token<'a> {
    K(Keyword),
    Op(&'a str),
    Ident(&'a str),
    Separator(char),
    Comment(&'a str),
    Err,
    A(Annotation),
}

impl Token<'_> {
    pub(crate) fn not_comment(&self) -> bool {
        match self {
            Token::Comment(_) => false,
            _ => true
        }
    }
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Op(s) => write!(f, "{}", s),
            Token::Separator(c) => write!(f, "{}", c),
            Token::Ident(s) => write!(f, "{}", s),
            Token::K(k) => match k {
                Keyword::Fn => write!(f, "fn"),
                Keyword::If => write!(f, "if"),
                Keyword::Else => write!(f, "else"),
                Keyword::While => write!(f, "while"),
                Keyword::For => write!(f, "for"),
                Keyword::In => write!(f, "in"),
                Keyword::Class => write!(f, "class"),
            },
            Token::Comment(s) => write!(f, "{}", s),
            Token::Err => write!(f, "Err"),
            Token::A(a) => match a {
                Annotation::SequenceEntrypoint => write!(f, "@SequenceEntrypoint"),
            },
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Keyword {
    Fn,
    If,
    Else,
    While,
    For,
    In,
    Class
}

pub type Spanned<T> = (T, Range<usize>);

pub type TokSpan<'a, 'b> = LocatedSpan<Token<'a>, (ParseState<'b>, usize)>;

pub trait FromStrSpan<'a, 'b> {
    fn from_strspan(token: Token<'a>, state: ParseState<'b>, span: Range<usize>) -> TokSpan<'a, 'b>;
}

impl<'a, 'b> FromStrSpan<'a, 'b> for TokSpan<'a, 'b> {
    #[inline]
    fn from_strspan(token: Token<'a>, state: ParseState<'b>, span: Range<usize>) -> TokSpan<'a, 'b> {
        unsafe{TokSpan::new_from_raw_offset(span.start, 0, token, (state, span.end-span.start))}
    }
}

impl<'a, 'b> ToRange for TokSpan<'a, 'b> {
    fn span(&self) -> Range<usize> {
        let start = self.location_offset();
        start..start+self.extra.1
    }

    #[allow(unused_variables)]
    fn consumed_span(&self, next_start: usize) -> Range<usize> {
        unimplemented!()
    }
}

impl<'a, 'b> ToRange for Tokens<'a, 'b> {
    fn span(&self) -> Range<usize> {
        let start = self.offset;
        let end = match self.tokens.len() {
            0 => start+1, // TODO: Find out why the "+1". Find out why self.tokens.len() would be 0 in the first place
            _ => {
                let end = self.tokens[self.tokens.len()-1];
                end.location_offset()+end.extra.1
            }
        };
        start..end
    }

    fn consumed_span(&self, next_start: usize) -> Range<usize> {
        let start = self.span().start;
        let mut end = start;
        for token in self.tokens {
            let tok_span = token.span();
            if tok_span.start >= next_start {
                break
            } else {
                end = tok_span.end
            }
        }

        start..end
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Tokens<'a, 'b> {
    pub tokens: &'a [TokSpan<'a, 'b>],
    offset: usize,
    pub state: ParseState<'a>
}

impl<'a, 'b> Tokens<'a, 'b> {
    pub fn new(tokens: &'a [TokSpan<'a, 'b>], state: ParseState<'a>) -> Tokens<'a, 'b> {
        Tokens { tokens, offset: 0, state }
    }
}

impl<'a, 'b> Display for Tokens<'a, 'b> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.tokens)
    }
}

impl<'a, 'b> Compare<Token<'a>> for Tokens<'a, 'b> {
    fn compare(&self, t: Token) -> CompareResult {
        if self.tokens.len() == 0 || *self.tokens[0].fragment() != t {
            CompareResult::Error
        } else {
            CompareResult::Ok
        }
    }

    fn compare_no_case(&self, t: Token) -> CompareResult {
        self.compare(t)
    }
}

impl<'a, 'b> InputIter for Tokens<'a, 'b> {
    type Item = TokSpan<'a, 'b>;

    type Iter = Enumerate<Self::IterElem>;

    type IterElem = Copied<Iter<'a, Self::Item>>;

    fn iter_indices(&self) -> Self::Iter {
        unimplemented!()
    }

    fn iter_elements(&self) -> Self::IterElem {
        unimplemented!()
    }

    fn position<P>(&self, predicate: P) -> Option<usize>
  where
    P: Fn(Self::Item) -> bool {
        self.tokens.iter().position(|b| predicate(*b))
    }

    fn slice_index(&self, count: usize) -> Result<usize, nom::Needed> {
        if self.tokens.len() >= count {
            Ok(count)
        } else {
        Err(Needed::new(count - self.tokens.len()))
        }
    }
}

impl<'a, 'b> InputLength for Tokens<'a, 'b> {
    #[inline]
    fn input_len(&self) -> usize {
        self.tokens.input_len()
    }
}

impl<'a> InputLength for Token<'a> {
    #[inline]
    fn input_len(&self) -> usize {
        1
    }
}

impl<'a, 'b> InputTake for Tokens<'a, 'b> {
    fn take(&self, count: usize) -> Self {
        Tokens::new(&self.tokens[0..count], self.state)
    }

    fn take_split(&self, count: usize) -> (Self, Self) {
        let (prefix, suffix) = self.tokens.split_at(count);
        let suf_offset = match suffix.len() {
            0 => match prefix.len() {
                0 => self.offset,
                _ => prefix[0].span().end
            },
            _ => suffix[0].span().start
        };
        (Tokens{tokens: suffix, offset: suf_offset, state: self.state}, Tokens{tokens: prefix, offset: self.offset, state: self.state})
    }
}
