#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

/// We emit a `[u8; 63usize]` padding field for this struct, which cannot derive
/// Debug/Hash because 63 is over the hard coded limit.
#[repr(C)]
#[repr(align(64))]
#[derive(Copy, Clone)]
pub struct NoDebug {
    pub c: ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_NoDebug() {
    assert_eq!(
        ::std::mem::size_of::<NoDebug>(),
        64usize,
        concat!("Size of: ", stringify!(NoDebug))
    );
    assert_eq!(
        ::std::mem::align_of::<NoDebug>(),
        64usize,
        concat!("Alignment of ", stringify!(NoDebug))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<NoDebug>() };
            let struct_ptr = &struct_instance as *const NoDebug;
            let field_ptr = std::ptr::addr_of!(struct_instance.c);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(NoDebug),
            "::",
            stringify!(c)
        )
    );
}
impl Default for NoDebug {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl ::std::cmp::PartialEq for NoDebug {
    fn eq(&self, other: &NoDebug) -> bool {
        self.c == other.c
    }
}
/// This should derive Debug/Hash/PartialEq/Eq because the padding size is less than the max derive
/// Debug/Hash/PartialEq/Eq impl for arrays. However, we conservatively don't derive Debug/Hash because
/// we determine Debug derive-ability before we compute padding, which happens at
/// codegen.
#[repr(C)]
#[repr(align(64))]
#[derive(Copy, Clone)]
pub struct ShouldDeriveDebugButDoesNot {
    pub c: [::std::os::raw::c_char; 32usize],
    pub d: ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_ShouldDeriveDebugButDoesNot() {
    assert_eq!(
        ::std::mem::size_of::<ShouldDeriveDebugButDoesNot>(),
        64usize,
        concat!("Size of: ", stringify!(ShouldDeriveDebugButDoesNot))
    );
    assert_eq!(
        ::std::mem::align_of::<ShouldDeriveDebugButDoesNot>(),
        64usize,
        concat!("Alignment of ", stringify!(ShouldDeriveDebugButDoesNot))
    );
    assert_eq!(
        {
            let struct_instance =
                unsafe { std::mem::zeroed::<ShouldDeriveDebugButDoesNot>() };
            let struct_ptr =
                &struct_instance as *const ShouldDeriveDebugButDoesNot;
            let field_ptr = std::ptr::addr_of!(struct_instance.c);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ShouldDeriveDebugButDoesNot),
            "::",
            stringify!(c)
        )
    );
    assert_eq!(
        {
            let struct_instance =
                unsafe { std::mem::zeroed::<ShouldDeriveDebugButDoesNot>() };
            let struct_ptr =
                &struct_instance as *const ShouldDeriveDebugButDoesNot;
            let field_ptr = std::ptr::addr_of!(struct_instance.d);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(ShouldDeriveDebugButDoesNot),
            "::",
            stringify!(d)
        )
    );
}
impl Default for ShouldDeriveDebugButDoesNot {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl ::std::cmp::PartialEq for ShouldDeriveDebugButDoesNot {
    fn eq(&self, other: &ShouldDeriveDebugButDoesNot) -> bool {
        self.c == other.c && self.d == other.d
    }
}
