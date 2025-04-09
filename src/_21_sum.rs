/// sum of 0..20, excluding multiples of 3 using for/if
fn sum_for_if() -> u32 {
    todo!("Implement")
}

// TODO implement sum_while_match
/// sum of 0..20, excluding multiples of 3 using while/match

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sum() {
        assert_eq!(sum_for_if(), 0);
        //assert_eq!(sum_while_match(), 0);
    }
}
