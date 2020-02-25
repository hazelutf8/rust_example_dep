pub mod ExampleDep {
    pub fn some_fn(val: u32) {
        debug_assert!(val == 5);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        ExampleDep::some_fn(5);
    }

    #[test]
    #[should_panic]
    fn it_fails() {
        ExampleDep::some_fn(0);
    }
}
