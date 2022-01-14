
use std::marker::PhantomData;

use crate::parser::*;

struct Many0<ISE: InputStateError, O, P: Parser<ISE, O>> {
    p: P,
    marker: PhantomData<(ISE, O)>,
}

impl<ISE: InputStateError, O, P: Parser<ISE, O>> Clone for Many0<ISE, O, P> {
    fn clone(&self) -> Self {
        Many0 {
            p: self.p.clone(),
            marker: PhantomData,
        }
    }
}

impl<ISE: InputStateError, O, P: Parser<ISE, O>> Parser<ISE, Vec<O>> for Many0<ISE, O, P> {
    fn parse(&self, state: &mut ISE::State, s: ISE::Input) -> PResult<ISE, Vec<O>> {
        let mut res = vec![];
        let mut s1 = s;
        loop {
            match self.p.parse(state, s1) {
                Ok((s, r)) => {
                    s1 = s;
                    res.push(r);
                },
                Err((s, _)) => {
                    break Ok((s, res))
                }
            }
        }
    }
}
pub fn many0<ISE: InputStateError, O>(p: impl Parser<ISE, O>) -> impl Parser<ISE, Vec<O>> {
    Many0 {
        p,
        marker: PhantomData,
    }
}
// pub fn many1<'a, S, O>(p: impl Parser<'a, S, O>) -> impl Parser<'a, S, Vec<O>> {
//     move |state: &mut S, s: &'a str| {
//         let mut res = vec![];
//         let mut s1 = s;
//         let tmp = p(state, s1);
//         if let Ok((s, r)) = tmp {
//             s1 = s;
//             res.push(r);
//         } else {
//             tmp?;
//         }
//         while let Ok((s, r)) = p(state, s1) {
//             s1 = s;
//             res.push(r);
//         }
//         Ok((s1, res))
//     }
// }

// pub fn times<'a, S: Clone, O>(p: impl Parser<'a, S, O>, n: usize) -> impl Parser<'a, S, Vec<O>> {
//     move |state: &mut S, s: &'a str| {
//         let initial = state.clone();
//         let mut res = vec![];
//         let mut s1 = s;
//         for _ in 0..n {
//             let r = p(state, s1);
//             if let Ok((s, r)) = r {
//                 s1 = s;
//                 res.push(r);
//             } else {
//                 *state = initial.clone();
//                 r?;
//             }
//         }
//         Ok((s1, res))
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;
    use super::super::pattern::*;

    #[test]
    fn test_many0() {
        let mut state = State::new();
        let parser = many0(pattern("a"));
        let res = parser.parse(&mut state, "aaaaa");
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.1, vec!["a", "a", "a", "a", "a"]);
        assert_eq!(res.0, "");
        assert_eq!(state.position(), 5);
    }
}