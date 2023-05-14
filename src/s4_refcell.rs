use crate::s3_cell::Cell;
use std::{
    cell::UnsafeCell,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

/// A mutable memory location with dynamically checked borrow rules
#[derive(Debug)]
pub struct RefCell<T> {
    _phantom: PhantomData<T>,
}

impl<T> RefCell<T> {
    pub fn new(value: T) -> RefCell<T> {
        RefCell {
            _phantom: PhantomData,
        }
    }
}

impl<T> RefCell<T> {
    /// Immutably borrows the wrapped value.
    ///
    /// The borrow lasts until the returned `Ref` exits scope. Multiple
    /// immutable borrows can be taken out at the same time.
    /// panic if already mutably borrowed
    pub fn borrow(&self) -> Option<&T> {
        /*
         * TODO
         */
        todo!()
    }

    /// Mutably borrows the wrapped value.
    /// panic if already borrowed.
    ///
    /// The borrow lasts until the returned `RefMut` or all `RefMut`s derived
    /// from it exit scope. The value cannot be borrowed while this borrow is
    /// active.
    pub fn borrow_mut(&self) -> Option<&mut T> {
        /*
         * TODO
         */
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "skip")]
    #[test]
    fn create_refcell() {
        let rf = RefCell::new(42);

        println!("{:?}", rf);
        assert_eq!(unsafe { *rf.inner.get() }, 42);
        assert_eq!(rf.state.get(), BorrowState::Unused);
    }

    #[cfg(feature = "skip")]
    #[test]
    fn borrow_many_times() {
        let rc = RefCell::new(42);
        let rc_ref1 = rc.borrow();
        let rc_ref2 = rc.borrow();

        assert_eq!(rc.state.get(), BorrowState::Shared(2));
    }

    #[cfg(feature = "skip")]
    #[test]
    fn borrow_mut_once() {
        let rc = RefCell::new(42);
        let rc_refmut = rc.borrow_mut();

        assert_eq!(rc.state.get(), BorrowState::Exclusive);
    }

    #[cfg(feature = "skip")]
    #[test]
    #[should_panic(expected = "already mutably borrowed")]
    fn borrow_panic() {
        let mut c = RefCell::new(42);

        let m = c.borrow_mut();
        let b = c.borrow(); // this causes a panic
    }

    #[cfg(feature = "skip")]
    #[test]
    fn borrow_mut_after_all_borrows_expires() {
        let rc = RefCell::new(42);
        {
            let rc_ref1 = rc.borrow();
            let rc_ref2 = rc.borrow();
        }
        let ref_mut = rc.borrow_mut();

        assert_eq!(rc.state.get(), BorrowState::Exclusive);
    }

    #[cfg(feature = "skip")]
    #[test]
    fn borrow_mut() {
        let c = RefCell::new("hello".to_owned());

        *c.borrow_mut() = "bonjour".to_owned();

        assert_eq!(&*c.borrow(), "bonjour");
    }

    #[test]
    fn refcell_demo() {
        use std::cell::{RefCell, RefMut};
        use std::collections::HashMap;
        use std::rc::Rc;

        let shared_map: Rc<RefCell<_>> = Rc::new(RefCell::new(HashMap::new()));
        // Create a new block to limit the scope of the dynamic borrow
        {
            let mut map: RefMut<_> = (*shared_map).borrow_mut();
            map.insert("africa", 92388);
            map.insert("kyoto", 11837);
            map.insert("piccadilly", 11826);
            map.insert("marbles", 38);
        }

        // Note that if we had not let the previous borrow of the cache fall out
        // of scope then the subsequent borrow would cause a dynamic thread panic.
        // This is the major hazard of using `RefCell`.
        let total: i32 = shared_map.borrow().values().sum();
        println!("{total}");
    }
}

mod refcell_usecase {

    /**
     * A Use Case for Interior Mutability: Mock Objects
     */
    pub trait Messenger {
        fn send(&self, msg: &str); // <--- self is immutable
    }

    pub struct LimitTracker<'a, T: Messenger> {
        messenger: &'a T,
        value: usize,
        max: usize,
    }

    impl<'a, T> LimitTracker<'a, T>
    where
        T: Messenger,
    {
        pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
            LimitTracker {
                messenger,
                value: 0,
                max,
            }
        }

        pub fn set_value(&mut self, value: usize) {
            self.value = value;

            let percentage_of_max = (self.value as f64) / (self.max as f64);

            if percentage_of_max >= 1.0 {
                self.messenger.send("Error: You are over your quota!");
            } else if percentage_of_max >= 0.9 {
                self.messenger
                    .send("Urgent warning: You've used up over 90% of your quota!");
            } else if percentage_of_max >= 0.75 {
                self.messenger
                    .send("Warning: You've used up over 75% of your quota!");
            }
        }
    }

    #[cfg(test)]
    mod failed_mocker_tests {
        use super::*;

        struct MockMessenger {
            sent_messages: Vec<String>,
        }

        impl MockMessenger {
            fn new() -> MockMessenger {
                MockMessenger {
                    sent_messages: vec![],
                }
            }
        }

        impl Messenger for MockMessenger {
            fn send(&self, message: &str) { // <-- cannot change it to mutable here :-(
                                            // self.sent_messages.push(String::from(message));
            }
        }

        #[test]
        fn it_sends_an_over_75_percent_warning_message() {
            let mock_messenger = MockMessenger::new();
            let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

            limit_tracker.set_value(80);

            let ref_messages = mock_messenger.sent_messages;
            assert_eq!(ref_messages.len(), 1);
            assert_eq!(
                ref_messages[0],
                "Warning: You've used up over 75% of your quota!"
            );
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::cell::RefCell;

        struct MockMessenger {
            sent_messages: RefCell<Vec<String>>,
        }

        impl MockMessenger {
            fn new() -> MockMessenger {
                MockMessenger {
                    sent_messages: RefCell::new(vec![]),
                }
            }
        }

        impl Messenger for MockMessenger {
            fn send(&self, message: &str) {
                self.sent_messages.borrow_mut().push(String::from(message));
            }
        }

        #[test]
        fn it_sends_an_over_75_percent_warning_message() {
            let mock_messenger = MockMessenger::new();
            let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

            limit_tracker.set_value(80);

            let ref_messages = mock_messenger.sent_messages.borrow();
            assert_eq!(ref_messages.len(), 1);
            assert_eq!(
                ref_messages[0],
                "Warning: You've used up over 75% of your quota!"
            );
        }
    }
}
