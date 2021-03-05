fn main() {
    println!("foo: {}", cfg!(foo));
    println!("pathdep::foo: {}", pathdep::foo());
    println!("gitdep::foo: {}", gitdep::foo());
}
