
pub mod parser;
pub mod location;

pub mod pattern;
pub mod position;
pub mod combinator;
pub mod repeat;


#[cfg(test)]
mod tests {
    use super::*;
    #[derive(Clone)]
    pub(crate) struct State {
        pub(crate) abs: usize,
        b: usize,
    }
    impl<'a> parser::ProvidePosition<&'a str> for State {
        type Position = usize;
        fn position(&self) -> usize {
            self.abs
        }
        fn shift(&mut self, s: &'a str) {
            self.abs += s.chars().count();
        }
    }
    impl parser::BackTrack for State {
        fn save(&mut self) {
            self.b = self.abs
        }
        fn backtrack(&mut self) {
            self.abs = self.b
        }
    }
    impl State {
        pub fn new() -> Self {
            State {
                abs: 0,
                b: 0
            }
        }
    }
}