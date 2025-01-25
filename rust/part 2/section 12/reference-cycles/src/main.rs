use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    next: Option<Rc<RefCell<Node>>>,
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("Dropping Node");
    }
}

fn main() {
    println!("Hello, world!");
    let a = Rc::new(RefCell::new(Node { next: None }));
    println!("a a: {:?}", Rc::strong_count(&a));

    let b = Rc::new(RefCell::new(Node { next: Some(Rc::clone(&a)) }));
    println!("b is created \n a count is: {:?}", Rc::strong_count(&a));
    println!("b is created : {:?}", Rc::strong_count(&b));

    let c = Rc::new(RefCell::new(Node { next: Some(Rc::clone(&b)) }));
    println!("c is created \n a count is: {:?}", Rc::strong_count(&a));
    println!("b count is: {:?}", Rc::strong_count(&b));

    println!("<<< --- >>>");
    (*a).borrow_mut().next = Some(Rc::clone(&c));
    println!("a count is: {:?}", Rc::strong_count(&a));
    println!("b count is: {:?}", Rc::strong_count(&b));
    println!("c count is: {:?}", Rc::strong_count(&c));
}
