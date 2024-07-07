//! A simple, single-dependency crate to benchmark the execution time of a block of code.
//!
//! # Examples
//!
//! ### Time single execution
//! ```
//! #[macro_use]
//! extern crate timethis;
//!
//! fn main() {
//!     let time_taken = timethis!({
//!         let mut x = Vec::new();
//!         for i in 0..1000 {
//!             x.push(i);
//!         }
//!     });
//!
//!     println!("Time taken for 1,000 pushes: {:?}", time_taken);
//! }
//! ```
//! 
//! ### Time multiple execution
//! ```
//! #[macro_use]
//! extern crate timethis;
//!
//! fn main() {
//!     let time_taken = timethis_loops!(10, {
//!         let mut x = Vec::new();
//!         for i in 0..1000 {
//!             x.push(i);
//!         }
//!     });
//!
//!     println!("Time taken for 10,000 pushes: {:?}", time_taken);
//! }
//! ```


#[macro_export]
/// Runs the block of code once and returns the execution time
/// in a [`std::time::Duration`].
macro_rules! timethis {
    ($block:block) => {{
        let start = std::time::Instant::now();
        $block
        start.elapsed()
    }};
}

#[macro_export]
/// Runs the block of code `n` times and returns the execution time
/// in a [`std::time::Duration`]
macro_rules! timethis_loops {
    ($nloops:expr, $block:block) => {{
        let n = $nloops;

        let start = std::time::Instant::now();
        for _ in 0..n {
            $block
        }

        start.elapsed()
    }};
}
