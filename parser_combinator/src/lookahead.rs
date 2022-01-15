
use crate::parser::*;

pub fn lookahead<ISE: InputStateError, O>(p: impl Parser<ISE, O>) -> impl Parser<ISE, O>
where
    ISE::Input: Clone,
{
    move |state: &mut ISE::State, s: ISE::Input| {
        let (_, res) = p.parse(state, s.clone())?;
        Ok((s, res))
    }
}

pub fn not<ISE: InputStateError, O>(p: impl Parser<ISE, O>) -> impl Parser<ISE, ()>
where
    ISE::Input: Clone,
{
    move |state: &mut ISE::State, s: ISE::Input| {
        match p.parse(state, s.clone()) {
            Ok(_) => Err((s, ISE::Error::from(ParseError("not".to_string(), state.position())))),
            Err(_) => Ok((s, ()))
        }
    }
}