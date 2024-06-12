fn main() {
    let arr = [2, 4, 8, 16];

    let mut n = 2;
    let nth = nth_item(&arr, n);
    let increased = increased_by_first_item(&arr, &mut n);

    let value = {
        let values = TwoValues::new(&arr[3], increased);

        assert_eq!(values.get_first(), &16);

        values.get_second()
    };

    assert_eq!(*value, 4);
    assert_eq!(*nth, 8);

    println!("*value - {}", *value);
    println!("*nth - {}", *nth);
}

// Removed the reference from the n parameter, as it's not needed to be a reference.
fn nth_item(data: &[usize], n: usize) -> &usize {
    &data[n]
}

fn increased_by_first_item<'a>(data: &[usize], n: &'a mut usize) -> &'a mut usize {
    *n += data[0];
    n
}

struct TwoValues<'a> {
    first: &'a usize,
    second: &'a usize,
}

// To fix the code by adding appropriate lifetime annotations,
// we need to ensure that references in the TwoValues struct have
// lifetimes tied to the references passed to the new function.

// In the TwoValues struct, I've added a lifetime 'a and applied
// it to both the first and second fields. This ensures that the
// references inside TwoValues have the same lifetime as the
// references passed to the new function. This way, the references
// remain valid for the lifetime of the data they point to.

impl<'a> TwoValues<'a> {
    pub fn new(first: &'a usize, second: &'a usize) -> Self {
        Self { first, second }
    }

    pub fn get_first(&self) -> &'a usize {
        self.first
    }

    pub fn get_second(&self) -> &'a usize {
        self.second
    }
}
