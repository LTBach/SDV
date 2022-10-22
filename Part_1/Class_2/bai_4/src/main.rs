fn main() {
    let mut v = vec![1, 2, 3];

    go(&mut v);

    // still need v here, so I can't pass ownership to the "go' method above
    println!("{}", v.len())
}

fn go(v: &mut Vec<i32>) {
    for i in v.iter() {
        println!("{}", i);
    }
    v.push(4);
}