#![cfg_attr(not(test), no_std)]

pub type Res<'a, T, E> = Option<Result<(T, &'a str), E>>;

pub trait Lex<'a> {
    type Token;
    type Error;

    fn lex(&mut self, _: &'a str) -> Res<'a, Self::Token, Self::Error>;
}

impl<'a, F, T, E> Lex<'a> for F
where
    F: FnMut(&'a str) -> Res<'a, T, E>,
{
    type Token = T;
    type Error = E;

    fn lex(&mut self, s: &'a str) -> Res<'a, T, E> {
        (self)(s)
    }
}

pub struct Lexer<'a, 'b, F: Lex<'a>> {
    rem: &'a str,
    f: &'b mut F,
}

impl<'a, 'b, F: Lex<'a>> Iterator for Lexer<'a, 'b, F> {
    type Item = Result<F::Token, F::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.rem.is_empty() {
            return None;
        }

        let ret;
        (ret, self.rem) = match self.f.lex(self.rem)? {
            Ok(x) => x,
            Err(x) => return Some(Err(x)),
        };
        Some(Ok(ret))
    }
}

pub fn lex<'a, 'b, F: Lex<'a>>(s: &'a str, f: &'b mut F) -> Lexer<'a, 'b, F> {
    Lexer { rem: s, f }
}
