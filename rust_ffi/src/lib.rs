use std::ffi::CStr;

#[repr(C)]
pub struct StringTest {
    name: *mut ::std::os::raw::c_char
}

#[repr(C)]
pub struct NestedTest {
    string_test: StringTest
}

#[repr(C)]
pub struct PointerTest {
    pointer: *const ::std::os::raw::c_void
}

#[repr(C)]
union TestUnion {
    number: f64,
    pointer: *const ::std::os::raw::c_void,
}

#[repr(C)]
pub struct UnionAndStringTest {
    test_union: TestUnion,
    string_test: StringTest
}

impl StringTest {
    fn name(&self) -> &str {
        let name = unsafe { CStr::from_ptr(self.name) };
        name.to_str().unwrap()
    }
}

#[link(name = "cpp_examples")]
extern {
    fn createStringTest() -> StringTest;
    fn createNestedTest() -> NestedTest;
    fn createPointerTest() -> PointerTest;
    fn createUnionAndStringTest() -> UnionAndStringTest;
}

fn create_string_test() -> Option<StringTest>
{
    unsafe {
        let res = createStringTest();
        Some(res)
    }
}

fn create_nested_test() -> Option<NestedTest>
{
    unsafe {
        let res = createNestedTest();
        Some(res)
    }
}

fn create_pointer_test() -> Option<PointerTest>
{
    unsafe {
        let res = createPointerTest();
        Some(res)
    }
}

fn create_union_and_string_test() -> Option<UnionAndStringTest>
{
    unsafe {
        let res = createUnionAndStringTest();
        Some(res)
    }
}

#[cfg(test)]
mod tests {
    use crate::{create_string_test, create_nested_test, create_pointer_test, StringTest, create_union_and_string_test};
    use std::ffi::CStr;

    #[test]
    fn test_string() {
        let res = create_string_test();
        let out = res.unwrap();
        let name = unsafe { CStr::from_ptr(out.name) };
        assert_eq!(name.to_str().unwrap(), "Betty");
        assert_eq!(out.name(), "Betty");
    }

    #[test]
    fn test_nested() {
        let res = create_nested_test();
        let out = res.unwrap();
        assert_eq!(out.string_test.name(), "Betty");
    }

    #[test]
    fn test_pointer() {
        let res = create_pointer_test();
        let out = res.unwrap();
        //let strTest = unsafe { out.pointer.as_ref() };
        //let casted = strTest.unwrap() as &StringTest;
        let ref_transmuted = unsafe {
            std::mem::transmute::<*const ::std::os::raw::c_void, &StringTest>(out.pointer)
        };
        assert_eq!(ref_transmuted.name(), "Betty");
    }

    #[test]
    fn test_union_and_string() {
        let res = create_union_and_string_test();
        let out = res.unwrap();
        assert_eq!(out.string_test.name(), "Betty");
    }
}
