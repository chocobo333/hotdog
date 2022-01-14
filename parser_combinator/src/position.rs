
use std::marker::PhantomData;
use std::ops::Range;

use crate::parser::*;


pub fn position<ISE, Pos>() -> impl Parser<ISE, Pos> 
where
    ISE: InputStateError<Position=Pos>,
    ISE::Error: From<ParseError<Pos>>
{
    move |state: &mut ISE::State, s: ISE::Input| {
        Ok((s, state.position()))
    }
}

struct Spanned<ISE: InputStateError, O, P: Parser<ISE, O>> {
    p: P,
    marker: PhantomData<(ISE, O)>
}

impl<ISE: InputStateError, O, P: Parser<ISE, O>> Clone for Spanned<ISE, O, P> {
    fn clone(&self) -> Self {
        Spanned {
            p: self.p.clone(),
            marker: PhantomData,
        }
    }
}

impl<ISE: InputStateError, O, P: Parser<ISE, O>> Parser<ISE, (O, Range<ISE::Position>)> for Spanned<ISE, O, P> {
    fn parse(&self, state: &mut ISE::State, s: ISE::Input) -> PResult<ISE, (O, Range<ISE::Position>)> {
        let start = state.position();
        let (s, res) = self.p.parse(state, s)?;
        let end = state.position();
        Ok((s, (res, start..end)))
    }
}

pub fn spanned<ISE, Pos, O>(p: impl Parser<ISE, O>) -> impl Parser<ISE, (O, Range<Pos>)> 
where
    ISE: InputStateError<Position=Pos>,
    ISE::Error: From<ParseError<Pos>>,
{
    Spanned {
        p,
        marker: PhantomData,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;
    use super::super::pattern::*;

    use super::super::combinator::*;
    #[test]
    fn test_position() {
        let mut state = State::new();
        let parser = pair(position(), pattern("a"));
        let res = parser.parse(&mut state, "abc");
        let res = res.unwrap();
        assert_eq!(res.1, (0, "a"));
        assert_eq!(res.0, "bc");
    }
    // #[test]
    // fn test_spanned() {
    //     let mut state = State{abs: 0};
    //     let parser = map(
    //             pair(
    //                 map_with_state(empty::<(&str, State), ()>, |s: &mut State, _: ()| s.position()),
    //                 map_with_state(pattern("a"), |s: &mut State, x| (x, s.position()))
    //             ),
    //             |(start, (res, end))| (res, start..end)
    //         );
    //     let res = parser.parse(&mut state, "abc");
    //     let res = res.unwrap();
    //     assert_eq!(res.1, ("a", Range{start: 0 as usize, end: 1 as usize}));
    //     assert_eq!(res.0, "bc");
    // }
    #[test]
    fn test_spanned2() {
        let mut state = State::new();
        let parser = spanned(pattern("a"));
        let res = parser.parse(&mut state, "abc");
        let res = res.unwrap();
        assert_eq!(res.1, ("a", Range{start: 0 as usize, end: 1 as usize}));
        assert_eq!(res.0, "bc");
    }
}