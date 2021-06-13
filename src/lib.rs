#![warn(clippy::all)]
#![warn(clippy::correctness)]
#![warn(clippy::style)]
#![warn(clippy::complexity)]
#![warn(clippy::perf)]

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
