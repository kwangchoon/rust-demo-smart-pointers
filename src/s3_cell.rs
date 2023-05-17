use std::cell::UnsafeCell;
use std::marker::PhantomData;
use std::mem;
use std::ptr;

#[derive(Debug)]
pub struct Cell<T> {
    inner: UnsafeCell<T>,
}

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Cell {
            inner: UnsafeCell::new(value),
        }
    }

    /// Returns a copy of the contained value.
    /// This operation requires T with `Copy` binding.
    pub fn get(&self) -> T
    where
        T: Copy,
    {
        // SAFETY: This can cause data races if called from a separate thread,
        // but `Cell` is `!Sync` so this won't happen.
        unsafe { *self.inner.get() }
    }

    /// Replaces the contained value with value, and returns the old contained value.
    /// Hint: use mem::replace
    pub fn replace(&self, value: T) -> T {
        // SAFETY: This can cause data races if called from a separate thread,
        // but `Cell` is `!Sync` so this won't happen.
        // Hint: use mem::replace

        mem::replace(unsafe { &mut *self.inner.get() }, value)
    }

    /// Sets the contained value while dropping old value.
    /// Hint: use self::replace and drop
    pub fn set(&self, value: T) {
        let _ = self.replace(value);
    }

    /// Takes the value of the cell, leaving Default::default() in its place.
    /// Hint: use self::replace and Default::default()
    pub fn take(&self) -> T
    where
        T: Default,
    {
        self.replace(Default::default())
    }

    /// Swaps the values of two Cells. Difference with std::mem::swap is that this
    /// function doesn’t require &mut reference.
    /// Hint: use ptr::swap
    pub fn swap(&self, other: &Cell<T>) {
        if ptr::eq(self, other) {
            return;
        }

        // SAFETY: This can be risky if called from separate threads, but `Cell`
        // is `!Sync` so this won't happen. This also won't invalidate any
        // pointers since `Cell` makes sure nothing else will be pointing into
        // either of these `Cell`s.

        unsafe {
            ptr::swap(self.inner.get(), other.inner.get());
        }
    }

    /// Unwraps the value.
    /// Hint: use UnsafeCell::into_inner
    pub fn into_inner(self) -> T {
        self.inner.into_inner()
    }

    /* More ... */
}

impl<T> Cell<T> {
    /// Returns a raw pointer to the underlying data in this cell.
    /// Hint:: use UnsafeCell::get
    pub fn as_ptr(&self) -> *mut T {
        self.inner.get()
    }

    /// Returns a mutable reference to the underlying data.
    /// This call borrows Cell mutably (at compile-time) which guarantees that we possess the only reference.
    /// Hint: use UnsafeCell::get_mut
    pub fn get_mut(&mut self) -> &mut T {
        self.inner.get_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn get() {
        let c = Cell::new(42);
        assert_eq!(c.get(), 42);
    }

    #[test]
    fn replace() {
        let cell = Cell::new(42);

        assert_eq!(cell.replace(10), 42);
        assert_eq!(cell.get(), 10);
    }

    #[test]
    fn set() {
        let c = Cell::new(42);
        c.set(1569);
        assert_eq!(c.get(), 1569);

        struct Wrapper<'a>(&'a str);

        impl Drop for Wrapper<'_> {
            fn drop(&mut self) {
                println!("Dropping {}", self.0);
            }
        }

        let c = Cell::new(Wrapper("cell"));
        c.set(Wrapper("new cell"));
    }

    #[test]
    fn take() {
        let c = Cell::new(5);
        let five = c.take();

        assert_eq!(five, 5);
        assert_eq!(c.get(), 0);
    }

    #[test]
    fn swap() {
        let c1 = Cell::new(42_i32);
        let c2 = Cell::new(1569_i32);

        c1.swap(&c2);
        assert_eq!(1569, c1.get());
        assert_eq!(42, c2.get());
    }

    #[test]
    fn into_inner() {
        let c = Cell::new(5);
        let five = c.into_inner();

        assert_eq!(five, 5);
        // println!("c: {:?}", c); // error[E0382]: use of moved value: `c` (c is moved into five)
    }

    #[test]
    fn as_ptr() {
        let c = Cell::new(5);

        let ptr: *mut i32 = c.as_ptr();
        unsafe { *ptr += 1 };
        assert_eq!(c.get(), 6);

        unsafe { ptr.write(42) };
        assert_eq!(c.get(), 42);
    }

    #[test]
    fn get_mut() {
        let mut c = Cell::new(5);
        *c.get_mut() += 1;

        assert_eq!(c.get(), 6);
    }

    #[test]
    fn cell_is_send() {
        let cell = Cell::new(5);

        let t1 = thread::spawn(move || {
            cell.set(7);
            println!("cell value: {}", cell.get());
        });

        t1.join().unwrap();
    }

    #[cfg(feature = "skip")]
    #[test]
    fn cell_is_not_sync() {
        // & cannot be shared between threads safely (!Sync)
        let cell = Cell::new(5);

        let t1 = thread::spawn(|| {
            cell.set(7);
            println!("cell value: {}", cell.get());
        });

        t1.join().unwrap();
    }
}

mod another {

    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn test() {
        #[derive(Debug)]
        enum List {
            Cons(Cell<i32>, Rc<List>),
            Nil,
        }
        use List::{Cons, Nil};

        let mut a = Rc::new(Cons(
            Cell::new(5),
            Rc::new(Cons(Cell::new(10), Rc::new(Nil))),
        ));
        println!("a after = {a:?}");
        match *a {
            Cons(ref head, _) => {
                head.replace(42);
            }
            Nil => {}
        }
        println!("a after = {:?}", a);
    }
}
