// ● Write the following functions

// – f1: accepts mutable reference to tuple with two u32s and bool flag. If flag is false, returns mutable
// reference to the first element in the tuple. If flag is true, returns mutable reference to the second
// element in the tuple.
pub fn f1(tuple: &mut (u32, u32, bool)) -> Option<&mut u32> {
    if tuple.2 {
        Some(&mut tuple.1)
    } else {
        Some(&mut tuple.0)
    }
}
// – f2: accepts mutable slice &mut [u32] and number n, returns mutable reference to the n-th element
// in the slice (counting from zero).
pub fn f2(slice: &mut [u32], n: usize) -> Option<&mut u32> {
    slice.get_mut(n)
}
// – f3: accepts slice &mut [u32] and number n, returns mutable reference to the n-th element from the
// end of the slice (counting from zero, i.e. 0 means the last element).
pub fn f3(slice: &mut [u32], n: usize) -> Option<&mut u32> {
    slice.get_mut(slice.len().saturating_sub(n + 1))
}
// – f4: accepts slice &[u32], partitions it into 4 equal (as much as possible) parts, and returns 4
// resulting slices
pub fn f4(slice: &[u32]) -> [&[u32]; 4] {
    let len = slice.len();
    let chunk_size = (len + 3) / 4;
    [
        &slice[0..chunk_size],
        &slice[chunk_size..2 * chunk_size],
        &slice[2 * chunk_size..3 * chunk_size],
        &slice[3 * chunk_size..],
    ]
}

// ● Write unit tests for all functions
#[cfg(test)]
mod tests {
    use crate::{f1, f2, f3, f4};
    // use super::*;
    #[test]
    fn test_f1_false() {
        let mut tuple = (10, 20, false);
        assert_eq!(f1(&mut tuple), Some(&mut 10));
    }

    #[test]
    fn test_f1_true() {
        let mut tuple = (10, 20, true);
        assert_eq!(f1(&mut tuple), Some(&mut 20));
    }

    #[test]
    fn test_f2() {
        let mut slice = [10, 20, 30, 40];
        assert_eq!(f2(&mut slice, 2), Some(&mut 30));
    }

    #[test]
    fn test_f3() {
        let mut slice = [10, 20, 30, 40];
        assert_eq!(f3(&mut slice, 1), Some(&mut 30));
    }

    #[test]
    fn test_f4() {
        let slice = [10, 20, 30, 40, 50, 60, 70, 80];
        let result = f4(&slice);
        assert_eq!(result[0], [10, 20]);
        assert_eq!(result[1], [30, 40]);
        assert_eq!(result[2], [50, 60]);
        assert_eq!(result[3], [70, 80]);
    }
}
