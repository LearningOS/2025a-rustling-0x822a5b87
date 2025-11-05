// options2.rs
//
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a
// hint.

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        let word = match optional_target {
            None => None,
            Some(w) => Some(w)
        };
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        while let Some(integer_option) = optional_integers.pop() {
            match integer_option {
                None => {
                    assert_eq!(cursor, 0);
                }
                Some(w) => {
                    assert_eq!(w, cursor as i8);
                    cursor -= 1;
                }
            }
        }
    }
}
