
use regex::Regex;

use crate::parser::*;


#[derive(Clone)]
struct Pattern<I, S: ProvidePosition<I>> {
    re: Regex,
    marker: [(I, S); 0]
}

impl<'a, S: ProvidePosition<&'a str> + Clone> Parser<(&'a str, S), &'a str> for Pattern<&'a str, S> {
    fn parse(&self, state: &mut S, s: &'a str) -> PResult<(&'a str, S), &'a str> {
        if let Some(mat) = self.re.find(s) {
            state.shift(mat.as_str());
            Ok((&s[mat.end()..], mat.as_str()))
        } else {
            Err((s, ParseError("".to_string(), state.position())))
        }
    }
}

pub fn pattern<'a, S: ProvidePosition<&'a str> + Clone>(pat: &'static str) -> impl Parser<(&'a str, S), &'a str> {
    let regex = "^".to_string() + pat;
    let re = Regex::new(&regex).unwrap();
    Pattern {
        re,
        marker: []
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    #[test]
    fn test_pattern() {
        let mut state = State::new();
        let pat = pattern("[a-zA-Z][a-zA-Z0-9]+");
        let res = pat.parse(&mut state, "abc32_");
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.1, "abc32");
        assert_eq!(res.0, "_");
    }
}