use nutype::nutype;

#[nutype(validate(max = 1024))]
#[derive(Default)]
pub struct Count(i32);

fn main() {}
