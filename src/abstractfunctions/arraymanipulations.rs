
pub fn index_eiminator(array: Vec<i32>, index: usize) -> Vec<i32> {
    let mut array = [11, 2, 3];
    let index_to_array = index;
    for i in index_to_array..array.len() - 1 {
        array[i] = array[i + 1]
    }
    array.to_vec()
}

pub fn find_index<T: PartialEq>(array: &[T], element: &T) -> Option<usize> {
    for (index, item) in array.iter().enumerate() {
        if item == element {
            return Some(index);
        }
    }
    None
}

pub fn purify_array<T: Eq + Copy>(arr: &[T]) -> Vec<T> {
    let mut result = Vec::new();
    for &elem in arr {
        if !result.contains(&elem) {
            result.push(elem);
        }
    }
    result
}


#[test]
fn test_purify_array() {
    // Test case with repeated elements
    let arr = [1, 2, 2, 3, 4, 4, 4, 5];
    let expected = vec![1, 2, 3, 4, 5];
    assert_eq!(purify_array(&arr), expected);

    // Test case with no repeated elements
    let arr = [1, 2, 3, 4, 5];
    let expected = vec![1, 2, 3, 4, 5];
    assert_eq!(purify_array(&arr), expected);

    // Test case with a single element
    let arr = [1];
    let expected = vec![1];
    assert_eq!(purify_array(&arr), expected);

    // Test case with an empty array
    let arr: [i32; 0] = [];
    let expected: Vec<i32> = vec![];
    assert_eq!(purify_array(&arr), expected);
}
