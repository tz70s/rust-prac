// The general compound type
// i.e. vector, string, hash map

fn main() {
    let v: Vec<i32> = vec![1, 2, 3, 4, 5, 5, 7, 4, 3, 2, 5];

    println!("\nThis is the exercise of general std compound type");
    println!("Including - vector, string, hash map\n");
    println!("The average is : {}", average_func(&v));
    println!("The mid is : {}", mid_func(&v));
    println!("The mode number is : {}", mode_func(&v));
}

// average
fn average_func(v: &Vec<i32>) -> i32 {
    let mut sum: i32 = 0;
    let mut count = 0;
    for &i in v {
        sum += i;
        count += 1;
    }
    sum / count
}

// mid number
fn mid_func(v: &Vec<i32>) -> i32 {
    // copy data from immutable to mutable vector
    let mut new_v = v.clone();
    let length = new_v.len();
    let mut temp;
    // bubblesort
    for i in 0..length-1 {
        for j in i+1..length {
            if new_v[i] > new_v[j] {
                temp = new_v[i];
                new_v[i] = new_v[j];
                new_v[j] = temp;
            }
        }
    }

    // print the mid number
    new_v[length/2]
}

// mode number
fn mode_func(v: &Vec<i32>) -> i32 {
    use std::collections::HashMap;

    let mut counts = HashMap::new();

    // generate counts of numbers
    for &i in v {
        let count = counts.entry(i).or_insert(0);
        *count += 1;
    }

    let mut max = 0;
    let mut mode = 0;

    for (&k, &v) in &counts {
        if max < v {
            max = v;
            mode = k;
        }
    }
    mode
}

