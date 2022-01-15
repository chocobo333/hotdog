
use crate::parser::*;


pub fn alt<ISE: InputStateError, O>(p1: impl Parser<ISE, O>, p2: impl Parser<ISE, O>) -> impl Parser<ISE, O>
where
    ISE::State: BackTrack,
{
    move |state: &mut ISE::State, s: ISE::Input| {
        let pos = state.position();
        state.save();
        match p1.parse(state, s) {
            Ok((s, res)) => Ok((s, res)),
            Err((s, _)) => {
                state.backtrack();
                match p2.parse(state, s) {
                    Ok((s, res)) => Ok((s, res)),
                    Err((s, _)) => {
                        state.backtrack();
                        Err((s, ISE::Error::from(ParseError("".to_string(), pos))))
                    }
                }
            }
        }
    }
}

pub fn pair<ISE: InputStateError, O1, O2>(p1: impl Parser<ISE, O1>, p2: impl Parser<ISE, O2>) -> impl Parser<ISE, (O1, O2)>
where
    ISE::Input: Clone,
    ISE::State: BackTrack
{
    move |state: &mut ISE::State, s: ISE::Input| {
        let (s1, res1) = p1.parse(state, s.clone())?;
        match p2.parse(state, s1) {
            Ok((s, res2)) => Ok((s, (res1, res2))),
            Err((_, err)) => {
                state.backtrack();
                Err((s, err))
            }
        }
    }
}
// pub fn triple<ISE: InputStateError, O1, O2, O3>(p1: impl Parser<ISE, O1>, p2: impl Parser<ISE, O2>, p3: impl Parser<ISE, O3>) -> impl Parser<ISE, (O1, O2, O3)> {
//     move |state: &mut ISE::State, s: ISE::Input| {
//         let (s, res1) = p1.parse(state, s)?;
//         let (s, res2) = p2.parse(state, s)?;
//         let (s, res3) = p3.parse(state, s)?;
//         Ok((s, (res1, res2, res3)))
//     }
// }

pub fn seq<ISE: InputStateError, O, P: Parser<ISE, O>>(parsers: impl IntoIterator<Item=P> + Clone) -> impl Parser<ISE, Vec<O>> 
where
    ISE::Input: Clone,
    ISE::State: BackTrack,
{
    move |state: &mut ISE::State, s: ISE::Input| {
        state.save();
        let mut res = vec![];
        let mut s1 = s;
        for p in parsers.clone() {
            let r = p.parse(state, s1.clone());
            if let Ok((s, r)) = r {
                s1 = s;
                res.push(r);
            } else {
                state.backtrack();
                r?;
            }
        }
        Ok((s1, res))
    }
}

pub fn preceded<ISE: InputStateError, O1, O2>(p1: impl Parser<ISE, O1>, p2: impl Parser<ISE, O2>) -> impl Parser<ISE, O2>
where
    ISE::Input: Clone,
{
    move |state: &mut ISE::State, s: ISE::Input| {
        let (s1, _) = p1.parse(state, s.clone())?;
        match p2.parse(state, s1) {
            Ok(res) => Ok(res),
            Err((_, err)) => Err((s, err))
        }
    }
}
pub fn terminated<ISE: InputStateError, O1, O2>(p1: impl Parser<ISE, O1>, p2: impl Parser<ISE, O2>) -> impl Parser<ISE, O1>
where
    ISE::Input: Clone,
{
    move |state: &mut ISE::State, s: ISE::Input| {
        let (s1, res) = p1.parse(state, s.clone())?;
        match p2.parse(state, s1) {
            Ok((s, _)) => Ok((s, res)),
            Err((_, err)) => Err((s, err))
        }
    }
}

pub fn map<ISE: InputStateError, O1, O2>(p: impl Parser<ISE, O1>, callback: impl Fn(O1) -> O2 + Clone) -> impl Parser<ISE, O2> {
    move |state: &mut ISE::State, s: ISE::Input| {
        let (s, res) = p.parse(state, s)?;
        Ok((s, callback(res)))
    }
}

pub fn map_with_state<ISE: InputStateError, O1, O2>(p: impl Parser<ISE, O1>, callback: impl Fn(&mut ISE::State, O1) -> O2 + Clone) -> impl Parser<ISE, O2> {
    move |state: &mut ISE::State, s: ISE::Input| {
        let (s, res) = p.parse(state, s)?;
        Ok((s, callback(state, res)))
    }
}

pub fn map_res<ISE:InputStateError, O1, O2>(p: impl Parser<ISE, O1>, callback: impl Fn(PResult<ISE, O1>) -> PResult<ISE, O2> + Clone) -> impl Parser<ISE, O2> {
    move |state: &mut ISE::State, s: ISE::Input| {
        let res = p.parse(state, s);
        callback(res)
    }
}

pub fn opt<ISE: InputStateError, O>(p: impl Parser<ISE, O>) -> impl Parser<ISE, Option<O>> {
    move |state: &mut ISE::State, s: ISE::Input| {
        match p.parse(state, s) {
            Ok((s, res)) => Ok((s, Some(res))),
            Err((s, _)) => Ok((s, None)),
        }
    }
}

pub fn empty<ISE: InputStateError, O: Default>() -> impl Parser<ISE, O> {
    |_: &mut ISE::State, s: ISE::Input| {
        Ok((s, O::default()))
    } 
}

pub fn separated1<ISE: InputStateError, O1, O2>(p: impl Parser<ISE, O1>, delimiter: impl Parser<ISE, O2>) -> impl Parser<ISE, Vec<O1>>
where
    ISE::Input: Clone,
    ISE::State: BackTrack,
{
    use crate::repeat::many0;
    map(
        pair(p.clone(), many0(preceded(delimiter.clone(), p.clone()))),
        |(x, xs)| {
            let mut res = vec![x];
            for e in xs {
                res.push(e);
            }
            res
        }
    )
}
pub fn separated0<ISE: InputStateError, O1, O2>(p: impl Parser<ISE, O1>, delimiter: impl Parser<ISE, O2>) -> impl Parser<ISE, Vec<O1>>
where
    ISE::Input: Clone,
    ISE::State: BackTrack,
{
    alt(separated1(p, delimiter), empty())
}

pub fn delimited<ISE: InputStateError, O1, O2, O3>(p1: impl Parser<ISE, O1>, p2: impl Parser<ISE, O2>, p3: impl Parser<ISE, O3>) -> impl Parser<ISE, O2>
where
    ISE::Input: Clone
{
    preceded(p1, terminated(p2, p3))
}
pub fn surrounded<ISE: InputStateError, O1, O2>(outer: impl Parser<ISE, O1>, inner: impl Parser<ISE, O2>) -> impl Parser<ISE, O2>
where
    ISE::Input: Clone
{
    delimited(outer.clone(), inner, outer)
}

impl<ISE: InputStateError, O1, O2, P1: Parser<ISE, O1>, P2: Parser<ISE, O2>> Parser<ISE, (O1, O2)> for (P1, P2)
where
    ISE::Input: Clone,
    ISE::State: BackTrack,
{
    fn parse(&self, state: &mut ISE::State, s: ISE::Input) -> PResult<ISE, (O1, O2)> {
        pair(self.0.clone(), self.1.clone()).parse(state, s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;
    use super::super::pattern::*;

    #[test]
    fn test_alt() {
        let parser = alt(pattern("a"), pattern("b"));
        let res = parser.parse(&mut (), "bbc");
        assert!(res.is_ok());
        assert_eq!(res.unwrap().1, "b");
    }
    #[test]
    fn test_pair() {
        let parser = pair(pattern("a"), pattern("b"));
        let res = parser.parse(&mut (), "abc");
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.1, ("a", "b"));
        assert_eq!(res.0, "c");

        let parser = (pattern("a"), pattern("b"));
        let res = parser.parse(&mut (), "abc");
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.1, ("a", "b"));
        assert_eq!(res.0, "c");
        // let parser = triple(pattern("a"), pattern("b"), pattern("c"));
        // let res = parser.parse(&mut (), "abc");
        // assert!(res.is_ok());
        // let res = res.unwrap();
        // assert_eq!(res.1, ("a", "b", "c"));
        // assert_eq!(res.0, "");
        // let mut state = State{abs: 0};
        // let parser = triple(position(), pattern("a"), position());
        // let res = parser.parse(&mut state, "abc");
        // assert!(res.is_ok());
        // let res = res.unwrap();
        // assert_eq!(res.1, (0, "a", 1));
        // assert_eq!(res.0, "bc");
    }
    #[test]
    fn test_seq() {
        let parser = seq(vec![pattern("a"), pattern("b")]);
        let res = parser.parse(&mut (), "abc");
        assert!(res.is_ok());
        assert_eq!(res.unwrap().1, vec!["a", "b"]);
    }
    #[test]
    fn test_preceded() {
        let parser = preceded(pattern("a"), pattern("b"));
        let res = parser.parse(&mut (), "abc");
        assert!(res.is_ok());
        assert_eq!(res.unwrap().1, "b");
    }
    #[test]
    fn test_terminated() {
        let parser = terminated(pattern("a"), pattern("b"));
        let res = parser.parse(&mut (), "abc");
        assert!(res.is_ok());
        assert_eq!(res.unwrap().1, "a");
    }
    #[test]
    fn test_map() {
        let mut state = State::new();
        let parser = map(pattern("[1-9][0-9]+"), |x| x.parse::<i32>().unwrap());
        let res = parser.parse(&mut state, "123");
        assert!(res.is_ok());
        assert_eq!(res.unwrap().1, 123);
    }
    #[test]
    fn test_map_with_state() {
        let mut state = State::new();
        let parser = map_with_state(pattern("[1-9][0-9]+"), |s: &mut State, x| (x.parse::<i32>().unwrap(), s.position()));
        let res = parser.parse(&mut state, "123");
        assert!(res.is_ok());
        assert_eq!(res.unwrap().1, (123, 3));
    }
    // #[test]
    // fn test_map_res() {
    //     // TODO: todo!
    //     {}
    // }
    #[test]
    fn test_opt() {
        let parser = opt(pattern("a"));
        let (s, res1) = parser.parse(&mut (), "abc").unwrap();
        let (_, res2) = parser.parse(&mut (), s).unwrap();
        assert_eq!(res1, Some("a"));
        assert_eq!(res2, None);
    }
    #[test]
    fn test_empty() {
        let num = map(pattern("[1-9][0-9]+"), |x| x.parse::<i32>().unwrap());
        let parser = alt(num, empty());
        let (s, res1) = parser.parse(&mut (), "123abc").unwrap();
        let (_, res2) = parser.parse(&mut (), s).unwrap();
        assert_eq!(res1, 123);
        assert_eq!(res2, 0);
    }
    #[test]
    fn test_separated1() {
        let parser = separated1(pattern("[a-z]"), pattern(","));
        let (s, res) = parser.parse(&mut (), "a,b,c,").unwrap();
        assert_eq!(res, vec!["a", "b", "c"]);
        assert_eq!(s, ",");
        let res = parser.parse(&mut (), "123");
        assert!(res.is_err())
    }
    #[test]
    fn test_separated0() {
        let parser = separated0(pattern("[a-z]"), pattern(","));
        let (s, res) = parser.parse(&mut (), "a,b,c,").unwrap();
        assert_eq!(res, vec!["a", "b", "c"]);
        assert_eq!(s, ",");
        let res = parser.parse(&mut (), "123");
        assert!(res.is_ok())
    }
    #[test]
    fn test_delimited() {
        let parser = delimited(pattern(r"\("), map(pattern("0|[1-9][0-9]*"), |x| x.parse().unwrap()), pattern(r"\)"));
        let (s, res): (_, usize) = parser.parse(&mut (), "(34)").unwrap();
        assert_eq!(res, 34);
        assert_eq!(s, "");
    }
    #[test]
    fn test_surrounded() {
        let parser = surrounded(pattern("\""), map(pattern("0|[1-9][0-9]*"), |x| x.parse().unwrap()));
        let (s, res): (_, i128) = parser.parse(&mut (), "\"34\"").unwrap();
        assert_eq!(res, 34);
        assert_eq!(s, "");
    }
}