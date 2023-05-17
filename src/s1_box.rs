/**
 * Using `Box<T>` to Store Data on the Heap:
 *
 * The most straightforward smart pointer is a box, whose type is written `Box<T>`.
 * Boxes allow you to store data on the heap rather than the stack. What remains
 * on the stack is the pointer to the heap data.
 *
 * Just like any owned value, when a box goes out of scope, it will be deallocated.
 * The deallocation happens both for the box (stored on the stack) and the data it
 * points to (stored on the heap).
 */
#[test]
fn box_pointer() {
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    //  b (stack) [  ---]-------> Point { x: 5, y: 10 } (heap)
    let b: Box<Point> = Box::new(Point { x: 5, y: 10 });

    println!("b (stack) = {:p}", std::ptr::addr_of!(b));
    println!("b (stack) = {:p}", &b);
    println!("b (heap)= {:p}", b);
    println!("b = {:?}", b);
}

/**
 * Using Box<T> Like a Reference: (Following the Pointer to the Value: `*`)
 *
 * We can rewrite the code to use a `Box<T>` instead of a reference (&); the dereference
 * operator (*) used on the `Box<T>` functions in the same way as the dereference operator
 * used on the reference:
 */

#[test]
fn use_just_like_references() {
    let x = 5;
    let ref_x = &x;
    let box_x = Box::new(x);

    assert_eq!(5, x);
    assert!(*ref_x == *box_x);
}

#[test]
fn moving_without_box() {
    use std::thread;

    let data = [42; 1024 * 10];

    println!("data@ = {:p}", std::ptr::addr_of!(data)); // &data

    let handle = thread::spawn(move || {
        println!("data@ = {:p}", std::ptr::addr_of!(data));
    });

    handle.join().unwrap();
}

/**
 * Moving the `Box<T>` Will Only Move the Data on the Stack:
 */
#[test]
fn moving_with_box() {
    use std::thread;

    let data = Box::new([42; 1024 * 10]);

    println!("data@ (stack) = {:p}", std::ptr::addr_of!(data));
    println!("data@ (heap) = {:p}", &*data);

    let handle = thread::spawn(move || {
        println!("data@ (stack) = {:p}", std::ptr::addr_of!(data));
        println!("data@ (heap) = {:p}", &*data);
    });

    handle.join().unwrap();
}

/**
 * Enabling Recursive Types with Boxes:
 *
 * Because boxes have a known size, we can enable recursive types by inserting a box
 * in the recursive type definition.
 */
#[cfg(feature = "skip")]
mod compile_error {
    //Doesn't compile.
    enum List {
        Cons(i32, List),
        Nil,
    }

    fn demo() {
        let list = Cons(1, Cons(2, Cons(3, Nil)));
    }
}

#[derive(Debug)]
enum List {
    Nil,
    Cons(i32, Box<List>),
}

use List::{Cons, Nil};

#[test]
fn cons_list() {
    //  |            |
    //  | Cons(1, *) |----> | Cons(2, *) | ---> | Cons(3, *) | ----> | Nil |
    //  |            |
    let list: List = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("list = {:?}", list);
}

#[test]
fn owning_trait_object() {
    fn random_animal(random_number: f64) -> Box<dyn Animal> {
        if random_number < 0.5 {
            Box::new(Dog)
        } else {
            Box::new(Cat)
        }
    }

    trait Animal {
        fn sound(&self) -> &str;
    }

    struct Dog;
    struct Cat;

    impl Animal for Dog {
        fn sound(&self) -> &str {
            "woof"
        }
    }

    impl Animal for Cat {
        fn sound(&self) -> &str {
            "meow"
        }
    }

    let animal: Box<dyn Animal> = random_animal(0.4);
    println!("{}", animal.sound());
}

#[test]
fn what_if_box_is_not_smart_pointer() {
    let b = Box::new(String::from("Hello"));
    let inner: String = *b; // Box<String> => String

    let b = Box::new(String::from("Hello"));
    // let inner_inner: &str = &*&*b; // Box<String> => String => &str
    let inner_inner: &str = &b; // Box<String> => String => &str
}

#[test]
fn what_is_going_on_here() {
    use std::ops::Deref;

    let b = Box::new(String::from("Hello"));

    let inner: &String = &b; // &Box<String> => &String
    let inner: &str = &b; // &Box<String> => &str ???
}

#[test]
fn auto_deref() {
    use std::ops::Deref;

    let b = Box::new(String::from("Hello"));

    // *b => *b.deref()

    // &A => &B    A(...(...(B(.. C(xxxx)..))))
    // &A => &C

    let inner: &String = &*b; // &Box<String> => &String: using auto-deref
    let inner: &String = &*(b.deref()); // &Box<String> => &String

    let inner: &String = &b; // &Box<String> => &String: using auto-deref
    let inner: &String = b.deref(); // &Box<String> => &String: using auto-deref

    let inner_inner: &str = &**b; // &Box<String> => &String => &str
    let inner_inner: &str = &*(*b.deref());
    let inner_inner: &str = &*((*b.deref()).deref());

    let inner_inner: &str = &b; // &Box<String> => &String => &str: using auto-deref twice
    let inner_inner: &str = b.deref().deref(); // &Box<String> => &String => &str: using auto-deref twice

    // If the result of `deref()` is a reference, `deref()` will be called again
    // as many times as necessary until a value of type T is obtained => "deref coercion".
}
/**
 * Implicit "Deref Coercions" with Functions and Methods:
 *
 * Deref coercion was added to Rust so that programmers writing function and method
 * calls don’t need to add as many explicit references and dereferences with & and *.
 * The deref coercion feature also lets us write more code that can work for either
 * references or smart pointers.
 *
 * The number of times that `Deref::deref` needs to be inserted is resolved at compile time,
 * so there is no runtime penalty for taking advantage of deref coercion!
 */

#[test]
fn deref_coercion() {
    use std::ops::Deref;

    fn hello(name: &str) {
        println!("Hello, {name}!");
    }

    let m = Box::new(String::from("Rust"));
    hello(&m); // &Box<String> => &String => &str

    // If there were no deref coercion, we would have to write the following:
    hello(&(*m)[..]);
}

/**
 * Std Box functions
 */
#[test]
fn into_raw() {
    let b = Box::new(String::from("Hello"));

    let ptr: *mut String = Box::into_raw(b); // b moved
    println!("ptr = {:?}", unsafe { &*ptr });

    // do something with ptr
    unsafe {
        ptr.replace(String::from("World"));

        let s = ptr.read();
        println!("s = {s:?}");

        ptr.write(String::from("Rustonean!"));
    }

    println!("ptr = {:?}", unsafe { &*ptr });
}

#[test]
fn implicit_coercion_from_ref_to_ptr() {
    /**
     * &mut T => *mut T => *const T => *mut T => &mut T
     */
    fn foo(x: *const i32) {
        println!("x = {:?}", unsafe { &*x })
    }

    let mut x = 32;
    let p = &mut x;
    foo(p);

    let p = &mut x as *mut i32;
    foo(p);
}

#[test]
fn test_from_raw() {
    let mut b = Box::new(String::from("Hello"));
    let p: *mut String = Box::into_raw(b); // b moved

    unsafe {
        println!("old = {:?}", p.replace(String::from("World")));
        let b: Box<String> = Box::from_raw(p);

        println!("b = {b:?}");
    }
}

// #[test]
// fn quiz() {
//     fn foo(s: &mut String) {
//         let kk: &mut String = s; // copy or move?? --> reborrowing!
//         println!("kk = {:?}", kk);

//         s.push_str("world!");
//         println!("s = {:?}", s);
//     }

//     fn bar(s: &String) {
//         let kk = s; // copy or move?
//         println!("kk = {:?}", kk);
//         println!("s = {:?}", s);
//     }

//     let mut x = String::from("Hello");

//     bar(&x); // 1. copy or borrow or move? & : Copy
//     foo(&mut x); // 2.  copy or borrow or move? &mut: Copy??????? ===> 'reborrowing'
// }

/**
 * Running Code on Cleanup with the `Drop` Trait:
 *
 * The second trait important to the smart pointer pattern is `Drop`, which lets you customize
 * what happens when a value is about to go out of scope. You can provide an implementation for
 * the `Drop` trait on any type, and that code can be used to release resources like files or
 * network connections. For example, when a `Box<T>` is dropped it will deallocate the space on
 * the heap that the box points to.
 *
 * You specify the code to run when a value goes out of scope by implementing the `Drop` trait.
 * The `Drop` trait requires you to implement one method named `drop` that takes a `&mut self`.
 *
 * Rust automatically called `drop` for us when our instances went out of scope, calling the
 * code we specified. Variables are dropped in the reverse order of their creation.
 */
struct CustomSmartPointer(String);

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.0);
    }
}

#[test]
fn drop_order1() {
    let c = CustomSmartPointer(String::from("first"));
    let d = CustomSmartPointer(String::from("second"));
}

#[test]
fn drop_order2() {
    struct Foo {
        x: CustomSmartPointer,
        y: CustomSmartPointer,
    }

    let foo = Foo {
        x: CustomSmartPointer(String::from("first")),
        y: CustomSmartPointer(String::from("second")),
    };
    println!("Leaving...");
}

/**
 * Dropping a Value Early with `std::mem::drop`
 *
 * Unfortunately, it’s not straightforward to disable the automatic drop functionality.
 * Rust doesn’t let you call the `Drop` trait’s `drop` method manually (to prevent double-free);
 * instead you have to call the `std::mem::drop` function provided by the standard library if you
 * want to force a value to be dropped before the end of its scope.
 */

#[test]
fn exlitcit_call_to_drop_is_not_allowed() {
    let c = CustomSmartPointer(String::from("some data"));
    // c.drop(); // error: explicit use of destructor method label: explicit destructor calls not allowed
}

#[test]
fn use_the_std_mem_drop_for_early_drop() {
    let c = CustomSmartPointer(String::from("some data"));
    std::mem::drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}
