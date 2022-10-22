fn vec_diff(left: &Vec<i32>, right: &Vec<i32>) -> Vec<i32> {
    let mut rv = Vec::new();
    for (l, r) in left.iter().zip(right.iter()) {
        rv.push(l - r);
    }
    return rv;
}

fn all(vec: &Vec<i32>, value: i32) -> bool {
    for elem in vec.iter() {
        if *elem != value {
            return false;
        }
    }
    return true;
}

fn main() {
    let v1 = vec![0, 1, 2];
    let v2 = vec![3, 4, 5];
    let diff = vec_diff(&v2, &v1);

    let diff2  = vec_diff(&diff, &v1);
    println!("{:?}", all(&diff, 3));
}