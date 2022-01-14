
pub trait ProvidePosition<I> {
    type Position;
    fn position(&self) -> Self::Position;
    fn shift(&mut self, s: I);
}
pub trait BackTrack {
    fn save(&mut self);
    fn backtrack(&mut self);
}

impl<I> ProvidePosition<I> for () {
    type Position = ();
    fn position(&self) -> () {
        ()
    }
    fn shift(&mut self, _: I) {}
}
impl BackTrack for () {
    fn save(&mut self) {}
    fn backtrack(&mut self) {}
}

// pub trait Parser<'a, S, O>: Fn(&mut S, &'a str) -> PResult<'a, O> + Clone {}
// impl <'a, S, O, T: Fn(&mut S, &'a str) -> PResult<'a, O> + Clone> Parser<'a, S, O> for T {}

pub trait InputStateError {
    type Input;
    type Position;
    type State: ProvidePosition<Self::Input, Position=Self::Position>;
    type Error: From<ParseError<Self::Position>>;
}
impl<I, S: ProvidePosition<I>> InputStateError for (I, S) {
    type Input = I;
    type Position = <S as ProvidePosition<I>>::Position;
    type State = S;
    type Error = ParseError<Self::Position>;
}
impl<I, S: ProvidePosition<I>, E: From<ParseError<<S as ProvidePosition<I>>::Position>>> InputStateError for (I, S, E) {
    type Input = I;
    type Position = <S as ProvidePosition<I>>::Position;
    type State = S;
    type Error = E;
}
#[derive(Debug)]
pub struct ParseError<Pos> (pub String, pub Pos);
pub type PResult<ISE, O> = Result<(<ISE as InputStateError>::Input, O), (<ISE as InputStateError>::Input, <ISE as InputStateError>::Error)>;
pub trait Parser<ISE: InputStateError, O>: Clone {
    fn parse(&self, state: &mut ISE::State, s: ISE::Input) -> PResult<ISE, O>;
}

impl<ISE: InputStateError, O, F: Fn(&mut ISE::State, ISE::Input) -> PResult<ISE, O> + Clone> Parser<ISE, O> for F {
    fn parse(&self, state: &mut ISE::State, s: ISE::Input) -> PResult<ISE, O> {
        self(state, s)
    }
}
// pub struct MyParser {
    //     line: usize,
//     character: usize,
//     abs: usize,

//     indent: Vec<usize>,
//     errs: Vec<ParseError>,
// }

// impl<'a> MyParser {
//     pub fn new() -> Self {
//         MyParser {
//             line: 0,
//             character: 0,
//             abs: 0,

//             indent: vec![0],
//             errs: vec![],
//         }
//     }
//     pub fn parse(&mut self, s: &'a str) -> PResult<'a, RTerm> {

//     }
//     fn program(&mut self, s: &'a str) -> PResult<'a, RTerm> {
//         separated1(statement, nodent)(s)
//     }
//     fn nodent(&mut self, s: &'a str) -> () {

//     }
// }
// fn newline<'a>(state: &mut MyParser, s: &'a str) -> PResult<'a, usize> {
//     let pat = pattern(r"[ ]+\n");
//     let tmp = many1(pat);
//     let (s, _) = tmp(state, s)?;
//     let (s, spaces) = sp0(state, s)?;
//     Ok((s, spaces))
// }

// fn sp0<'a>(state: &mut MyParser, s: &'a str) -> PResult<'a, usize> {
//     // let pat = pattern(r"[ ]*");
//     let (s, res) = pattern(r"[ ]*")(state, s)?;
//     Ok((s, res.len()))
// }
// fn sp1<'a>(state: &mut MyParser, s: &'a str) -> PResult<'a, usize> {
//     let pat = pattern(r"[ ]+");
//     let (s, res) = pat(state, s)?;
//     Ok((s, res.len()))
// }
// impl ProvidePosition for MyParser {
//     fn position(&self) -> Position {
//         Position::new(self.line, self.character, self.abs)
//     }
// }