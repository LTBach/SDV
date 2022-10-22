fn main(){

}

pub fn iter_num(num: i32) -> bool {

    let num_str = num.to_string();
    let chars = num_str.chars(); // <-- move occurs because `chars` has type `Chars<'_>`, which does not implement the `Copy` trait
    let chars_2 = num_str.chars();
    let len =  chars.count();     // <-- `chars` moved due to this method call

    println!("Len = {:?}", len);

    for c in chars_2 {             // <-- ❌ "value used here after move": chars
        println!(">>> {:?}", c);
    }

    return true;
}