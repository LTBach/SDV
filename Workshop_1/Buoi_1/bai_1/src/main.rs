use std::io;

fn main() {
    let mut n: String = String::new();
    
    println!("Input the number of array 1:");
    io::stdin().read_line(&mut n).expect("Fail to read line 0");
    let n: i32 = n.trim().parse().unwrap();
    let mut arr1: Vec<i32> = Vec::new();

    for _ in 0..n {
        let mut tmp: String = String::new();
        io::stdin().read_line(&mut tmp).expect("Fail to read line");
        arr1.push(tmp.trim().parse().expect("Fail to parse"));
    }
    println!("{:?}", arr1);

    let mut m: String = String::new();

    println!("Input the number of array 2:");
    io::stdin().read_line(&mut m).expect("Fail to read line");
    let m: i32 = m.trim().parse().unwrap();
    let mut arr2: Vec<i32> = Vec::new();

    for _ in 0..m {
        let mut tmp: String = String::new();
        io::stdin().read_line(&mut tmp).expect("Fail to read line");
        arr2.push(tmp.trim().parse().expect("Fail to parse"));
    }
    println!("{:?}", arr2);

    let mut check: bool = true; 
    for i in 0..arr1.len() {
        check = true;
        for j in 0..arr2.len() {
            if arr1[i+j] != arr2[j] {
                check = false;
                break;
            }
        }
        if check {
            break;
        }
    }
    println!("The array 1 {}contain array 2.", if check {
        ""
    } else {
        "doesn't "
    });
}
