pub use ascii;
pub use itertools;
pub use lazy_static;
pub use lcmx;
pub use maplit;
pub use ndarray;
pub use peg;
pub use regex;

use std::time::Instant;

pub fn time_execution<T>(name: &str, f: impl Fn() -> T) -> T {
    let before = Instant::now();
    let result = f();
    println!("{} elapsed time: {:.2?}", name, before.elapsed());
    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
