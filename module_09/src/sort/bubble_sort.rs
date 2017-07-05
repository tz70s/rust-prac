// bubble sort

pub fn sort(v: &mut [i32]) {
    let length = v.len();
    let mut temp;

    for i in 0..length-1 {
        for j in 0..length-i-1 {
            if v[j] > v[j+1] {
                temp = v[j];
                v[j] = v[j+1];
                v[j+1] = temp;
            }
        }
    }

}