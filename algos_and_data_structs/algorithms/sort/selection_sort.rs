// This is a pure rust implementation of the selection sort algorithm
// For manual testing run:
// rustc selection_sort.rs
// ./selection_sort

/// Pure implementation of the selection sort algorithm in Rust.
///
/// # Arguments
/// 
/// * `collection` - Some mutable ordered collection with heterogeneous
/// comparable items inside.
///
/// # Returns
/// * `collection` - Same collection ordered by ascending
pub fn selection_sort(collection: &mut [i32]) {
    for i in 0..collection.len() {
        let mut least = i;
    
        for j in i..collection.len() {
            if collection[j] < collection[least] {
                least = j;
            }
        }
        if least != i {
            collection.swap(i, least); 
        }
    }
}

fn main() {
    let mut vector = Vec::new();
    let mut empty_vector = Vec::new();

    vector.push(0);
    vector.push(5);
    vector.push(3);
    vector.push(2);
    vector.push(2);

    println!("Vector: {:?}", vector); 
    selection_sort(&mut vector);
    println!("Sorted vector: {:?}", vector);

    selection_sort(&mut empty_vector);
    println!("Empty vector: {:?}", empty_vector);
    selection_sort(&mut empty_vector);

}
