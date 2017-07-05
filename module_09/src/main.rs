// simple practice of rust-by-example
// Ch9 - module

// structure - 
// mod sort and nested selection sort mod

mod sort;

fn main() {
    println!("Hello, world!");

    let mut arr = [1, 3, 5, 4, 2, 8, 13, 6];
    
    // original arr
    println!("original arr : {:?}", arr);

    // selection sort
    sort::selection_sort::sort(&mut arr[..]);
    println!("selection sorted arr : {:?}\n", arr);

    let mut arr_another = [1, 3, 5, 4, 2, 8, 13, 6];
    
    // original arr
    println!("original arr : {:?}", arr_another);

    // bubble sort
    sort::bubble_sort::sort(&mut arr_another[..]);
    println!("bubble sorted arr : {:?}\n", arr_another);
}
