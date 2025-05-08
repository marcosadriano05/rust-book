use std::{cell::RefCell, rc::Rc};

use smart_pointers::exec_rc;

// Rc + RefCell
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    exec_rc();

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("{a:?}");
    println!("{b:?}");
    println!("{c:?}");
}
