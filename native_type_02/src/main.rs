// simple practice of rust-by-example
// ch2 of native types

fn main() {
    println!("Hello, world!");

    println!("The usage of matrix transformation\n");
    matrix_transformation();

    println!("The usage of array & slice\n");
    use_array();
}

// practice of tuple
// implement matrix transformation

fn matrix_transformation() {
    let matrix = ((1, 2), (3, 4));

    println!("Original matrix : {:?}", matrix);

    println!("Reverse matrix : {:?}", reverse_matrix(&matrix));
}

fn reverse_matrix(m: &((i32, i32), (i32, i32))) -> ((i32, i32), (i32, i32)) {
    let ((a, b), (c, d)) = *m;
	((a, c), (b, d))
}
// practice of array and slice

fn use_array() {
    // explicit declaration
    let short_array: [i32; 5] = [1, 2, 3, 4, 5];

    // explicit declaration
    let long_array: [i32; 100] = [0; 100];

    // print from first to the fourth
    use_slice(&short_array[0..3]);

    // print from first to the tenth
    use_slice(&long_array[..9]);
}

fn use_slice(s: &[i32]) {
    println!("The slice is : {:?}", s);
}
