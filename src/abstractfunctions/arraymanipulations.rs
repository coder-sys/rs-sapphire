

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
