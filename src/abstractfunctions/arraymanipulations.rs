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
//match index {
//    Some(i) =>{
//        println!("Element {} found at index {}", element_to_find, i);
//    },
//    None => println!("Element {} not found in the array.", element_to_find),
//};
pub fn purify_array<T: Eq + Copy>(arr: &[T]) -> Vec<T> {
    let mut result = Vec::new();
    for &elem in arr {
        if !result.contains(&elem) {
            result.push(elem);
        }
    }
    result
}
