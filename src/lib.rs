#![cfg_attr(test, no_std)]

pub trait Lex {
    type Token;
    type Error;

    fn lex<'a>(&mut self, _: &'a str) -> Result<(Self::Token, &'a str), Self::Error>;
}

impl<F, T, E> Lex for F
where
    F: for<'a> FnMut(&'a str) -> Result<(T, &'a str), E>,
{
    type Token = T;
    type Error = E;

    fn lex<'a>(&mut self, s: &'a str) -> Result<(Self::Token, &'a str), Self::Error> {
        (self)(s)
    }
}

pub struct TokenIter<'a, 'b, F: Lex> {
    rem: &'a str,
    f: &'b mut F,
}

impl<'a, 'b, F: Lex> Iterator for TokenIter<'a, 'b, F> {
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

pub fn lex<'a, 'b, F: Lex>(s: &'a str, f: &'b mut F) -> TokenIter<'a, 'b, F> {
    TokenIter { rem: s, f }
}
