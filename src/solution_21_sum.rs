pub fn sum_for_if() -> u32 {
    let mut result = 0;
    for i in 0..20 {
        if i % 3 == 0 {
            continue;
        }
        result += i;
    }
    result
}

pub fn sum_while_match() -> u32 {
    let mut result = 0;
    let mut i = 0;
    while i < 20 {
        match i % 3 {
            0 => (),
            _ => result += i,
        }
        i += 1;
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sum() {
        assert_eq!(sum_for_if(), 127);
        assert_eq!(sum_while_match(), 127);
    }
}
