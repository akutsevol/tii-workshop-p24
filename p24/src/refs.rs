// ● Write the following functions

// – f1: accepts mutable reference to tuple with two u32s and bool flag. If flag is false, returns mutable
// reference to the first element in the tuple. If flag is true, returns mutable reference to the second
// element in the tuple.
pub fn f1() {
    println!("f1");
}
// – f2: accepts mutable slice &mut [u32] and number n, returns mutable reference to the n-th element
// in the slice (counting from zero).
pub fn f2() {
    println!("f2");
}
// – f3: accepts slice &mut [u32] and number n, returns mutable reference to the n-th element from the
// end of the slice (counting from zero, i.e. 0 means the last element).
pub fn f3() {
    println!("f3");
}
// – f4: accepts slice &[u32], partitions it into 4 equal (as much as possible) parts, and returns 4
// resulting slices
pub fn f4() {
    println!("f4");
}

// ● Write unit tests for all functions
#[cfg(test)]
mod tests {
    // use super::*;
    #[test]
    fn test_f1() {
        // let mut field = TicTacField::new();
        // assert_eq!(field.make_move(0, 0, Player::X), Ok(()));
        // assert_eq!(field.make_move(0, 0, Player::Y), Err(TicTacError::InvalidMove));
        // assert_eq!(field.make_move(3, 0, Player::X), Err(TicTacError::OutOfBounds));
    }
    #[test]
    fn test_f2() {
    }
    #[test]
    fn test_f3() {
    }
    #[test]
    fn test_f4() {
    }
}