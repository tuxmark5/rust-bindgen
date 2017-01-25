/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


pub const foo_THIS: foo = 0;
pub const foo_SHOULD_BE: foo = 1;
pub const foo_A_CONSTANT: foo = 2;
pub type foo = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct bar {
    pub this_should_work: foo,
}
#[test]
fn bindgen_test_layout_bar() {
    assert_eq!(::std::mem::size_of::<bar>() , 4usize);
    assert_eq!(::std::mem::align_of::<bar>() , 4usize);
}
impl Clone for bar {
    fn clone(&self) -> Self { *self }
}