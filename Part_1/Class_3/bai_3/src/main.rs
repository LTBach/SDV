fn main() {
    let a = A {p: Some("p".to_string())};
    a.a();
}

#[derive(Clone)]
struct A {
    p: Option<String>
}

impl A {
    fn a(self) -> Self {
        Self::b(&self.p.clone().unwrap());
        self
    }
    fn b(b: &str) {
        print!("b: {}", b)
    }
}