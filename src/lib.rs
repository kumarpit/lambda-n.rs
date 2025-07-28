#[cfg(test)]
mod tests {
    use lambda_n::lambda_n;

    #[test]
    fn it_works() {
        assert_eq!(lambda_n!(3 => __1 + __2 + __3)(1, 2, 3), 6);
    }

    /// Tests a closure with no arguments (a "thunk").
    #[test]
    fn test_zero_args() {
        let get_42 = lambda_n!(0 => 42);
        assert_eq!(get_42(), 42);
    }

    /// Tests a closure with a single argument.
    #[test]
    fn test_single_arg() {
        let square = lambda_n!(1 => __1 * __1);
        assert_eq!(square(8), 64);
    }

    /// Tests that the closure works with non-integer types like &str.
    #[test]
    fn test_different_types() {
        let greet = lambda_n!(2 => format!("Hello, {} and {}!", __1, __2));
        assert_eq!(greet("Alice", "Bob"), "Hello, Alice and Bob!");
    }

    /// Tests that the closure can capture variables from its environment.
    #[test]
    fn test_env_capture() {
        let prefix = "LOG: ";
        let logger = lambda_n!(1 => format!("{}{}", prefix, __1));
        assert_eq!(logger("File not found"), "LOG: File not found");
    }

    /// Tests storing the closure in a variable with an explicit function pointer type.
    #[test]
    fn test_storing_in_variable() {
        let subtract: fn(f64, f64) -> f64 = lambda_n!(2 => __1 - __2);
        assert_eq!(subtract(10.5, 3.0), 7.5);
    }

    /// Tests nesting the macro to create a higher-order function.
    #[test]
    fn test_nested_macro() {
        // Creates a function that takes one argument (`multiplier`)
        // and returns a new function that multiplies its own argument by `multiplier`.
        let make_multiplier = lambda_n!(1 => {
            let multiplier = __1;
            lambda_n!(move 1 => multiplier * __1) // Moves ownership of `multiplier`
        });

        let times_10 = make_multiplier(10);
        assert_eq!(times_10(7), 70);

        let times_5 = make_multiplier(5);
        assert_eq!(times_5(4), 20);
    }
}
