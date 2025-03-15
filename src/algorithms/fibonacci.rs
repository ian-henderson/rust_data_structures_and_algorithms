#![allow(unused)]

fn fibonacci_hof() -> impl FnMut() -> Option<u8> {
    let mut a: Option<u8> = Some(0);
    let mut b: Option<u8> = Some(1);

    move || {
        let result = a;
        a = b;
        b = result.and_then(|x| b.and_then(|y| x.checked_add(y)));
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIBONACCI_SERIES: [u8; 14] = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233];

    #[test]
    fn hof_should_calculate_series() {
        let mut f = fibonacci_hof();
        for i in FIBONACCI_SERIES {
            assert_eq!(f(), Some(i));
        }
        // max size of u8 is 255, should overflow here
        assert_eq!(f(), None);
    }
}
