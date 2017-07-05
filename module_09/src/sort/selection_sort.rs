// selection sort module

// sort existed array
pub fn sort(v: &mut [i32]) {
    let length = v.len();

    let mut temp;
    for i in 0..length-1 {
        for j in i+1..length {
            if v[i] > v[j] {
                temp = v[i];
                v[i] = v[j];
                v[j] = temp;
            }
        }
    }
}