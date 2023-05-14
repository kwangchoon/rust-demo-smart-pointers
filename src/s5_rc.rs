use crate::delim;
use crate::s3_cell::Cell;
use crate::s4_refcell::RefCell;
use std::{fmt, marker::PhantomData, ops::Deref, ptr::NonNull};

/**
 * Rc<T>, the Reference Counted Smart Pointer
 *
 * You have to enable multiple ownership explicitly by using the Rust type `Rc<T>`,
 * which is an abbreviation for reference counting. The `Rc<T>` type keeps track of
 * the number of references to a value to determine whether or not the value is
 * still in use. If there are zero references to a value, the value can be cleaned
 * up without any references becoming invalid.
 *
 * Note that `Rc<T>` is only for use in single-threaded scenarios.
 */

#[derive(Debug)]
struct Rc<T: fmt::Debug> {
    _phantom: PhantomData<T>,
}

impl<T: fmt::Debug> Rc<T> {
    fn new(value: T) -> Self {
        /*
         * Todo
         */
        todo!()
    }

    fn strong_count(this: &Self) -> usize {
        /*
         * TODO
         */
        todo!()
    }
}

/*
 * TODO: Implement `Clone`
 */

/*
 * TODO: Implement `Deref`
 */

/*
 * TODO: Implement `Drop`
 */

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}
use self::List::{Cons, Nil};

#[cfg(feature = "skip")]
#[test]
fn rc_test1() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));

    println!("a: {:?}", a);
    println!("b: {:?}", b);
    println!("c: {:?}", c);
}

#[cfg(feature = "skip")]
#[test]
fn rc_test2() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    assert_eq!(Rc::strong_count(&a), 1);

    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    assert_eq!(Rc::strong_count(&a), 2);
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
        assert_eq!(Rc::strong_count(&a), 3);
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    assert_eq!(Rc::strong_count(&a), 2);
}

#[cfg(feature = "skip")]
#[test]
fn rc_test3() {
    let rc = Rc::new(RefCell::new(String::from("hello")));
    rc.borrow_mut().push_str(", world");

    assert_eq!(rc.borrow().as_str(), "hello, world");
}

mod std_rc_demo {
    use crate::delim;
    use std::cell::RefCell;
    use std::rc::{Rc, Weak};

    #[derive(Debug)]
    struct Node {
        value: i32,
        next: Option<Rc<RefCell<Node>>>,
        head: Option<Weak<RefCell<Node>>>,
    }

    impl Drop for Node {
        fn drop(&mut self) {
            println!("Dropping {}", self.value);
        }
    }

    /**
     * Weak is used for break refernce cycle. A use case for Weak: a tree could use Rc
     * from parent to children, and Weak pointer from children to their parents. 
     * Calling `upgrade` on the Weak pointer returns an Option<Rc<T>>.
     *
     * Rc vs Weak
     * Rc strong reference counting. Reference cycle could cause the memory never be deallocated.
     * Weak weak reference counting that holds a non-owning reference to the allocated memory.
     */
    #[rustfmt::skip]
    #[test]
    fn rc_weak_demo() {
        // a
        let a = Rc::new(RefCell::new(Node { value: 1, next: None, head: None, }));
        println!("a strong count: {:?}, weak count: {:?}", Rc::strong_count(&a), Rc::weak_count(&a));
        delim!();

        // b --> a
        let b = Rc::new(RefCell::new(Node { value: 2, next: Some(Rc::clone(&a)), head: None, }));
        println!("a strong count: {:?}, weak count: {:?}", Rc::strong_count(&a), Rc::weak_count(&a));
        println!("b strong count: {:?}, weak count: {:?}", Rc::strong_count(&b), Rc::weak_count(&b));
        delim!();

        // c ---> b ---> a
        let c = Rc::new(RefCell::new(Node { value: 3, next: Some(Rc::clone(&b)), head: None, }));
        println!("a strong count: {:?}, weak count: {:?}", Rc::strong_count(&a), Rc::weak_count(&a));
        println!("b strong count: {:?}, weak count: {:?}", Rc::strong_count(&b), Rc::weak_count(&b));
        println!("c strong count: {:?}, weak count: {:?}", Rc::strong_count(&c), Rc::weak_count(&c));
        delim!();

        // Creates a reference cycle
        // c --> b --> a
        // ^           |
        // +-----------+
        a.borrow_mut().head = Some(Rc::downgrade(&c));
        println!("a strong count: {:?}, weak count: {:?}", Rc::strong_count(&a), Rc::weak_count(&a));
        println!("b strong count: {:?}, weak count: {:?}", Rc::strong_count(&b), Rc::weak_count(&b));
        println!("c strong count: {:?}, weak count: {:?}", Rc::strong_count(&c), Rc::weak_count(&c));
        delim!();

        println!("a {:?}", &a);
        println!("b {:?}", &b);
        println!("c {:?}", &c);
    }

    #[test]
    fn rc_cycle_demo() {
        #[derive(Debug)]
        struct Node {
            next: Option<Rc<RefCell<Node>>>,
        }

        impl Drop for Node {
            fn drop(&mut self) {
                println!("Dropping Node ...");
            }
        }

        // c --> b --> a
        // ^           |
        // +-----------+
        let a = Rc::new(RefCell::new(Node { next: None }));
        let b = Rc::new(RefCell::new(Node {
            next: Some(Rc::clone(&a)),
        }));
        let c = Rc::new(RefCell::new(Node {
            next: Some(Rc::clone(&b)),
        }));

        // Creates a reference cycle
        a.borrow_mut().next = Some(Rc::clone(&c));
        println!("a count: {:?}", Rc::strong_count(&a));
        println!("b count: {:?}", Rc::strong_count(&b));
        println!("c count: {:?}", Rc::strong_count(&c));

        // Print a will casue stack overlfow
        println!("c {:?}", &c);
    }
}
