// This is a pure rust implementation of the quick sort algorithm
// For manual testing run:
// rustc quick_sort.rs
// ./quick_sort
//
// Complexity: O(nlogn)
// Note: sorts in place, requires little additional memory. O(n^2) comparisons
// in the worst case.

/// Pure implementation of the quick sort algorithm in Rust.
///
/// # Arguments
/// 
/// * `collection` - Some mutable ordered collection with heterogeneous
/// comparable items inside.
/// * `l` - Index of start of collection.
/// * `h` - Index of end of collection.
///
/// # Returns
/// * `collection` - Same collection ordered by ascending order.
pub fn quick_sort(collection: &mut [i32], l: usize, h: usize) {
    if l < h {
        let p: usize = partition(collection, l, h);
        quick_sort(collection, l, p);
        quick_sort(collection, p+1, h);
    }
}


/// Pure implementation of the quick sort algorithm in Rust.
///
/// # Arguments
/// 
/// * `collection` - Some mutable ordered collection with heterogeneous
/// comparable items inside.
///
/// * `l` - Index of start of collection.
/// * `h` - Index of end of collection.
///
/// # Returns
/// * `collection` - Same collection ordered by ascending order.
pub fn partition(collection: &mut [i32], l: usize, h: usize) -> usize {
    let p_value = collection[l];
    let mut i = l as i32 - 1;
    let mut j = h as i32 + 1;
    loop { 
        i += 1;
        while collection[i as usize] < p_value { 
            i += 1; 
        }
        
        j -= 1;
        while collection[j as usize] > p_value { 
            j -= 1; 
        }
        
        if i >= j { break; }
        collection.swap(i as usize, j as usize);
    }
    j as usize
}
        

fn main() {
    let mut vector = Vec::new();
    vector.push(0);
    vector.push(5);
    vector.push(3);
    vector.push(2);
    vector.push(2);
    vector.push(-1);
    vector.push(1000);

    println!("Vector: {:?}", vector); 
    let length = vector.len() - 1;
    quick_sort(&mut vector, 0, length);
    println!("Vector: {:?}", vector); 
}
