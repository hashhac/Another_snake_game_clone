use std::fmt;
struct Val<T> {
    val: T
}
impl Val<i32> {
    fn new(val: i32) -> Self {
        Val { val:val }
    }
}
impl std::fmt::Display for Val<i32> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f,"{}",self.val)
}
}
fn main(){
    let x = Val::new(10);
    let y: Option<i32> = Some(10);
    println!("We got a new value y with type and value : {:?}", y);
    println!("We got a new value x with type and value : {} ", x );
}