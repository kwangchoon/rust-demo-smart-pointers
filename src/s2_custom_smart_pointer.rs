/**
 * Defining Custom Smart Pointers
 * 1. Creating a Smart Pointer by Using a Tuple Struct
 * 2. Treating a Type Like a Reference by Implementing the `Deref` Trait
 * 3. Treating a Type Like a Reference by Implementing the `DerefMut` Trait
 * 4. Running Code on Cleanup with the `Drop` Trait
 * 5. Enabling `Deref` Coercion with `AsRef`
 */
use std::fmt::Debug;

#[derive(Debug)]
struct MyBox<T: Debug>(T);

impl<T: Debug> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

#[test]
fn my_box_creation() {
    let mbox = MyBox::new(42);

    println!("{:?}", mbox);
}

/**
 * Treating Smart Pointers Like Regular References with the `Deref` Trait:
 *
 * Implementing the `Deref` trait allows you to customize the behavior of the
 * dereference operator *. By implementing `Deref` in such a way that a smart pointer
 * can be treated like a regular reference, you can write code that
 * operates on references and use that code with smart pointers too.
 */

use std::ops::Deref;

/*
 * TODO: define `Deref` for `MyBox`
 */

#[cfg(feature = "skip")]
#[test]
fn deref_for_custom_smart_pointer() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    println!("x = {}", *y.deref());
}

#[cfg(feature = "skip")]
#[test]
fn create_smart_pointer() {
    fn hello(name: &str) {
        println!("Hello, {name}!");
    }

    let m = Box::new(String::from("Rust"));
    hello(&m);

    let m = MyBox::new(String::from("Rust"));
    hello(&(*m.0)[..]); // in case we don't have the Deref coercion
                        // hello(&m);
}

#[cfg(feature = "skip")]
#[test]
fn using_deref_custom_smart_pointer() {
    let x = 42;
    let y = MyBox::new(MyBox::new(x));

    println!("inner = {:?}", *y);
    println!("x = {:?}", **y);
}

#[cfg(feature = "skip")]
#[test]
fn test_cascading_auto_deref_custom_smart_pointer() {
    fn foo(value: &i32) {
        println!("value = {}", *value);
    }

    let y = MyBox::new(MyBox::new(42));

    // method call
    foo(&y);

    // explicit assignment
    let derefed_x: &i32 = &y;
    println!("deferred_x = {derefed_x}");
}

/*
 * TODO: define `Drop` for `MyBox`
 */

#[test]
fn drop_test_for_smart_pointer() {
    let mbox = MyBox::new(String::from("Rust"));
}

#[test]
fn drop_test_when_shadowed() {
    let mut x = MyBox::new(String::from("Rust"));

    x = MyBox::new(String::from("Rust Rocks"));

    std::thread::sleep(std::time::Duration::from_millis(2000));
}

#[test]
fn cascading_drops_for_smart_pointer() {
    let mbox = MyBox::new(MyBox::new(String::from("Rust")));
}

/*
 * TODO: define `AsRef` for `MyBox`
 */

#[cfg(feature = "skip")]
#[test]
fn as_ref_for_custom_smart_pointer() {
    let mbox = MyBox::new(String::from("Rust"));

    let ref_t: &str = mbox.as_ref();
    println!("{}", ref_t);
}

#[cfg(feature = "skip")]
#[test]
fn as_ref_for_cascading_custom_smart_pointer() {
    let mbox = MyBox::new(MyBox::new(String::from("Rust")));

    let into_ref = MyBox::as_ref(&mbox);
    // let into_ref: &str = mbox.as_ref();
    println!("{:?}", into_ref);
}

/**
 * Example: `String` or `&str`
 *
 * Did you ever have had a function where you wanted to accept a string as a parameter?
 *
 * You might then also have asked yourself should you accept a string reference (as in
 * `&str`) or an owned string (as in `String`)?
 *
 * So why not have both?!
 * At this very point `AsRef<str>` comes in very handy, because both types `str` and `String`
 * implement this trait.
 */
mod as_ref_demos {
    #[test]
    fn take_both_str_reference_and_string_reference() {
        fn take_string(s: &str) {
            println!("take_string called with s = {s}");
        }

        take_string(&String::from("Rust"));
        take_string("Rust");
        // take_string(String::from("Rust")); // No type coercion
    }

    #[test]
    fn take_str_reference_and_string_reference_and_string1() {
        fn take_string(s: impl Into<String>) {
            let s = s.into();
            println!("take_string called with s = {s}");
        }

        take_string(&String::from("Rust"));
        take_string("Rust");
        take_string(String::from("Rust")); 
    }

    #[test]
    fn take_str_reference_and_string_reference_and_string2() {
        fn take_string<S: AsRef<str>>(s: S) {
            let s: &str = s.as_ref();
            println!("take_string called with s = {s}");
        }

        let s = String::from("Hello, World!");
        take_string(&s);
        take_string::<&str>(s.as_ref());
        take_string(s);
    }

    /**
     * Example: wrapper type
     */
    pub struct Envelope {
        letter: String,
    }

    impl AsRef<str> for Envelope {
        fn as_ref(&self) -> &str {
            // here we up-call to the `AsRef<str>` implementation for String
            self.letter.as_ref()
        }
    }

    #[test]
    fn struct_as_ref() {
        let a_letter = Envelope {
            letter: "a poem".to_string(),
        };

        println!("this is a letter: {}", a_letter.as_ref());
    }

    /**
     * Example: a composed type
     */
    struct Weight {
        weight: f32,
        unit: String,
    }

    impl Weight {
        /// Weight in Tons that is 157.47 stones
        pub fn from_tons(weight: f32) -> Self {
            Self {
                weight,
                unit: "t".to_string(),
            }
        }

        /// Weight in Stones
        pub fn from_stones(weight: f32) -> Self {
            Self {
                weight,
                unit: "st".to_string(),
            }
        }
    }

    /**
     * So how we can actually get our hand on the data inside?
     *
     * As you also have seen how `AsRef` can be useful to get access to inner data
     * of structs without having to provide accessory methods or public fields.
     */
    impl AsRef<str> for Weight {
        fn as_ref(&self) -> &str {
            &self.unit
        }
    }

    impl AsRef<f32> for Weight {
        fn as_ref(&self) -> &f32 {
            &self.weight
        }
    }

    #[test]
    fn get_access_to_inner_data_of_struct() {
        let a_ton = Weight::from_tons(1.3);

        let tons: &f32 = a_ton.as_ref();
        let unit: &str = a_ton.as_ref();

        println!("a weight of {tons}{unit}");

        let tons = a_ton.as_ref() as &f32;
        let unit = a_ton.as_ref() as &str;
        println!("a weight of {tons}{unit}");

        let tons = <Weight as AsRef<f32>>::as_ref(&a_ton);
        let unit = <Weight as AsRef<str>>::as_ref(&a_ton);
        println!("a weight of {tons}{unit}");
    }
}

///
/// The `Borrow` trait is used to represent borrowed data.
/// the `AsRef` trait is used for type conversion.
///
/// In Rust, it is common to provide different type representations for
/// different use cases for different semantics.
///
/// Choose `Borrow` when you want to abstract over different kinds of borrowing,
/// or when you’re building a data structure that treats owned and borrowed values
/// in equivalent ways, such as hashing and comparison.
///
/// Choose `AsRef` when you want to convert something to a reference directly, and
/// you’re writing generic code.
///
mod borrow_demo {
    use std::{
        borrow::Borrow,
        cell::RefCell,
        cmp,
        hash::{Hash, Hasher},
        rc::Rc,
    };

    #[derive(Debug)]
    struct MyMap<K, V> {
        map: Vec<(K, V)>,
    }

    impl<K, V> MyMap<K, V>
    where
        K: Hash + cmp::Eq,
    {
        fn new() -> Self {
            Self { map: Vec::new() }
        }

        fn add(&mut self, key: K, value: V) {
            self.map.push((key, value));
        }

        fn get<Q>(&self, key: &Q) -> Option<&V>
        where
            K: Borrow<Q>,
            Q: Hash + cmp::Eq + ?Sized,
        {
            for (k, v) in &self.map {
                if k.borrow() == key {
                    return Some(v);
                }
            }
            None
        }
    }

    #[test]
    fn test() {
        let mut map = MyMap::new();

        map.add(String::from("apple"), 3);
        map.add(String::from("kiwi"), 1);
        map.add(String::from("orange"), 5);

        assert_eq!(map.get("apple"), Some(&3));
        assert_eq!(map.get(&String::from("orange")), Some(&5));
    }
}
