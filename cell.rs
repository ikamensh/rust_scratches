use std::cell::Cell;

struct Incrementer <'a> {
    target: &'a Cell<i32>,
    step: i32
}

impl Incrementer <'_> {
    fn increment(&self){
        self.target.set(self.target.get()+self.step)
    }
}


fn main() {
    let v = Cell::new(1);

    // Both incrementers can mutate the v. This can't be achieved without a Cell.
    let i1 = Incrementer{target: &v, step: 1 };
    let i2 = Incrementer{target: &v, step: 2 };


    println!("{:?}", v);
    i1.increment();
    println!("{:?}", v);
    i2.increment();
    println!("{:?}", v);

}
