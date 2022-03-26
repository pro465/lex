#![cfg_attr(not(test), no_std)]

pub type Res<'a, T, E> = Result<(T, &'a str), E>;

pub trait Lex {
    type Token;
    type Error;

    fn lex<'a>(&mut self, _: &'a str) -> Res<'a, Self::Token, Self::Error>;
}

impl<F, T, E> Lex for F
where
    F: for<'a> FnMut(&'a str) -> Result<'a, T, E>,
{
    type Token = T;
    type Error = E;

    fn lex<'a>(&mut self, s: &'a str) -> Res<'a, Self::Token, Self::Error> {
        (self)(s)
    }
}

pub struct Lexer<'a, 'b, F: Lex> {
    rem: &'a str,
    f: &'b mut F,
}

impl<'a, 'b, F: Lex> Iterator for Lexer<'a, 'b, F> {
    type Item = Result<F::Token, F::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.rem.is_empty() {
            return None;
        }

        let ret;
        (ret, self.rem) = self.f.lex(self.rem).ok()?;
        Some(Ok(ret))
    }
}

pub fn lex<'a, 'b, F: Lex>(s: &'a str, f: &'b mut F) -> Lexer<'a, 'b, F> {
    Lexer { rem: s, f }
}
