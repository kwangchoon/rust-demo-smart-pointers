#![allow(unused)]

mod s1_box;
mod s2_custom_smart_pointer;
mod s3_cell;
mod s4_refcell;
mod s5_rc;

#[macro_export]
macro_rules! delim {
    () => {
        println!("{}", "-".repeat(50));
    };
    ($len:expr) => {
        println!("{}, " - ".repeat($len)");
    };
}
