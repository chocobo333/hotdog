
extern crate dsl;
extern crate parser_combinator;

pub use parser_combinator::*;


#[cfg(test)]
mod tests {
    use super::*;
    use super::dsl::*;

    fn second<A, B>((_, b): (A, B)) -> B {b}
    
    // parser!(
    //     State {
    //         indent: vec![0],
    //         errs: vec![],
    //     },
    //     Rule {
    //         newline: "([ ]*\n)+" + sp0 @ second @ len
    //         sp0: "[ ]*"
    //         sp1: "[ ]+"

    //     }
    // );
}