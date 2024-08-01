/// Calculates square root of an integer without using any exponent function.
///
/// # Arguments
///
/// * `x` - A non-negative integer.
///
/// # Returns
///
/// * Square root of `x` rounded down to the nearest integer.
///
/// # Complexity
///
/// * Time Complexity: O(log x)
///   * Binary search on a max search space of x / 2, hence O(log x).
/// * Space Complexity: O(1)
///   * Constant space as we use a fixed number of integer variables.
pub fn sqrt(x: u32) -> u32 {
    if x == 0 {
        return 0;
    } else if x == 1 {
        return 1;
    }

    let mut start = 1;
    let mut end = x / 2;
    let mut mid;

    while start <= end {
        mid = (start + end) / 2;

        let mid_squared = mid * mid;

        if mid_squared == x {
            return mid;
        } else if mid_squared > x {
            end = mid - 1;
        } else {
            start = mid + 1;
        }
    }

    end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sqrt_8() {
        assert_eq!(sqrt(8), 2);
    }

    #[test]
    fn sqrt_4() {
        assert_eq!(sqrt(4), 2);
    }

    #[test]
    fn sqrt_2() {
        assert_eq!(sqrt(2), 1);
    }
}
