#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
pub struct __BindgenUnionField<T>(::std::marker::PhantomData<T>);
impl<T> __BindgenUnionField<T> {
    #[inline]
    pub fn new() -> Self {
        __BindgenUnionField(::std::marker::PhantomData)
    }
    #[inline]
    pub unsafe fn as_ref(&self) -> &T {
        ::std::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_mut(&mut self) -> &mut T {
        ::std::mem::transmute(self)
    }
}
impl<T> ::std::default::Default for __BindgenUnionField<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
impl<T> ::std::clone::Clone for __BindgenUnionField<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new()
    }
}
impl<T> ::std::marker::Copy for __BindgenUnionField<T> {}
impl<T> ::std::fmt::Debug for __BindgenUnionField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("__BindgenUnionField")
    }
}
impl<T> ::std::hash::Hash for __BindgenUnionField<T> {
    fn hash<H: ::std::hash::Hasher>(&self, _state: &mut H) {}
}
impl<T> ::std::cmp::PartialEq for __BindgenUnionField<T> {
    fn eq(&self, _other: &__BindgenUnionField<T>) -> bool {
        true
    }
}
impl<T> ::std::cmp::Eq for __BindgenUnionField<T> {}
#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq, Eq)]
pub struct A {
    pub c: ::std::os::raw::c_uint,
    pub named_union: A__bindgen_ty_1,
    pub __bindgen_anon_1: A__bindgen_ty_2,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq, Eq)]
pub struct A_Segment {
    pub begin: ::std::os::raw::c_int,
    pub end: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_A_Segment() {
    assert_eq!(
        ::std::mem::size_of::<A_Segment>(),
        8usize,
        concat!("Size of: ", stringify!(A_Segment))
    );
    assert_eq!(
        ::std::mem::align_of::<A_Segment>(),
        4usize,
        concat!("Alignment of ", stringify!(A_Segment))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<A_Segment>() };
            let struct_ptr = &struct_instance as *const A_Segment;
            let field_ptr = std::ptr::addr_of!(struct_instance.begin);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(A_Segment),
            "::",
            stringify!(begin)
        )
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<A_Segment>() };
            let struct_ptr = &struct_instance as *const A_Segment;
            let field_ptr = std::ptr::addr_of!(struct_instance.end);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(A_Segment),
            "::",
            stringify!(end)
        )
    );
}
impl Clone for A_Segment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq, Eq)]
pub struct A__bindgen_ty_1 {
    pub f: __BindgenUnionField<::std::os::raw::c_int>,
    pub bindgen_union_field: u32,
}
#[test]
fn bindgen_test_layout_A__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<A__bindgen_ty_1>(),
        4usize,
        concat!("Size of: ", stringify!(A__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<A__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(A__bindgen_ty_1))
    );
}
impl Clone for A__bindgen_ty_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq, Eq)]
pub struct A__bindgen_ty_2 {
    pub d: __BindgenUnionField<::std::os::raw::c_int>,
    pub bindgen_union_field: u32,
}
#[test]
fn bindgen_test_layout_A__bindgen_ty_2() {
    assert_eq!(
        ::std::mem::size_of::<A__bindgen_ty_2>(),
        4usize,
        concat!("Size of: ", stringify!(A__bindgen_ty_2))
    );
    assert_eq!(
        ::std::mem::align_of::<A__bindgen_ty_2>(),
        4usize,
        concat!("Alignment of ", stringify!(A__bindgen_ty_2))
    );
}
impl Clone for A__bindgen_ty_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[test]
fn bindgen_test_layout_A() {
    assert_eq!(
        ::std::mem::size_of::<A>(),
        12usize,
        concat!("Size of: ", stringify!(A))
    );
    assert_eq!(
        ::std::mem::align_of::<A>(),
        4usize,
        concat!("Alignment of ", stringify!(A))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<A>() };
            let struct_ptr = &struct_instance as *const A;
            let field_ptr = std::ptr::addr_of!(struct_instance.c);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!("Offset of field: ", stringify!(A), "::", stringify!(c))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<A>() };
            let struct_ptr = &struct_instance as *const A;
            let field_ptr = std::ptr::addr_of!(struct_instance.named_union);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(A),
            "::",
            stringify!(named_union)
        )
    );
}
impl Clone for A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq, Eq)]
pub struct B {
    pub d: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq, Eq)]
pub struct B_Segment {
    pub begin: ::std::os::raw::c_int,
    pub end: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_B_Segment() {
    assert_eq!(
        ::std::mem::size_of::<B_Segment>(),
        8usize,
        concat!("Size of: ", stringify!(B_Segment))
    );
    assert_eq!(
        ::std::mem::align_of::<B_Segment>(),
        4usize,
        concat!("Alignment of ", stringify!(B_Segment))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<B_Segment>() };
            let struct_ptr = &struct_instance as *const B_Segment;
            let field_ptr = std::ptr::addr_of!(struct_instance.begin);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(B_Segment),
            "::",
            stringify!(begin)
        )
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<B_Segment>() };
            let struct_ptr = &struct_instance as *const B_Segment;
            let field_ptr = std::ptr::addr_of!(struct_instance.end);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(B_Segment),
            "::",
            stringify!(end)
        )
    );
}
impl Clone for B_Segment {
    fn clone(&self) -> Self {
        *self
    }
}
#[test]
fn bindgen_test_layout_B() {
    assert_eq!(
        ::std::mem::size_of::<B>(),
        4usize,
        concat!("Size of: ", stringify!(B))
    );
    assert_eq!(
        ::std::mem::align_of::<B>(),
        4usize,
        concat!("Alignment of ", stringify!(B))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<B>() };
            let struct_ptr = &struct_instance as *const B;
            let field_ptr = std::ptr::addr_of!(struct_instance.d);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!("Offset of field: ", stringify!(B), "::", stringify!(d))
    );
}
impl Clone for B {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum StepSyntax {
    Keyword = 0,
    FunctionalWithoutKeyword = 1,
    FunctionalWithStartKeyword = 2,
    FunctionalWithEndKeyword = 3,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq)]
pub struct C {
    pub d: ::std::os::raw::c_uint,
    pub __bindgen_anon_1: C__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq)]
pub struct C__bindgen_ty_1 {
    pub mFunc: __BindgenUnionField<C__bindgen_ty_1__bindgen_ty_1>,
    pub __bindgen_anon_1: __BindgenUnionField<C__bindgen_ty_1__bindgen_ty_2>,
    pub bindgen_union_field: [u32; 4usize],
}
#[repr(C)]
#[derive(Debug, Default, Copy, PartialEq)]
pub struct C__bindgen_ty_1__bindgen_ty_1 {
    pub mX1: f32,
    pub mY1: f32,
    pub mX2: f32,
    pub mY2: f32,
}
#[test]
fn bindgen_test_layout_C__bindgen_ty_1__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<C__bindgen_ty_1__bindgen_ty_1>(),
        16usize,
        concat!("Size of: ", stringify!(C__bindgen_ty_1__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<C__bindgen_ty_1__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(C__bindgen_ty_1__bindgen_ty_1))
    );
    assert_eq!(
        {
            let struct_instance =
                unsafe { std::mem::zeroed::<C__bindgen_ty_1__bindgen_ty_1>() };
            let struct_ptr =
                &struct_instance as *const C__bindgen_ty_1__bindgen_ty_1;
            let field_ptr = std::ptr::addr_of!(struct_instance.mX1);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(C__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(mX1)
        )
    );
    assert_eq!(
        {
            let struct_instance =
                unsafe { std::mem::zeroed::<C__bindgen_ty_1__bindgen_ty_1>() };
            let struct_ptr =
                &struct_instance as *const C__bindgen_ty_1__bindgen_ty_1;
            let field_ptr = std::ptr::addr_of!(struct_instance.mY1);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(C__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(mY1)
        )
    );
    assert_eq!(
        {
            let struct_instance =
                unsafe { std::mem::zeroed::<C__bindgen_ty_1__bindgen_ty_1>() };
            let struct_ptr =
                &struct_instance as *const C__bindgen_ty_1__bindgen_ty_1;
            let field_ptr = std::ptr::addr_of!(struct_instance.mX2);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(C__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(mX2)
        )
    );
    assert_eq!(
        {
            let struct_instance =
                unsafe { std::mem::zeroed::<C__bindgen_ty_1__bindgen_ty_1>() };
            let struct_ptr =
                &struct_instance as *const C__bindgen_ty_1__bindgen_ty_1;
            let field_ptr = std::ptr::addr_of!(struct_instance.mY2);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(C__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(mY2)
        )
    );
}
impl Clone for C__bindgen_ty_1__bindgen_ty_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy, Hash, PartialEq, Eq)]
pub struct C__bindgen_ty_1__bindgen_ty_2 {
    pub mStepSyntax: StepSyntax,
    pub mSteps: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_C__bindgen_ty_1__bindgen_ty_2() {
    assert_eq!(
        ::std::mem::size_of::<C__bindgen_ty_1__bindgen_ty_2>(),
        8usize,
        concat!("Size of: ", stringify!(C__bindgen_ty_1__bindgen_ty_2))
    );
    assert_eq!(
        ::std::mem::align_of::<C__bindgen_ty_1__bindgen_ty_2>(),
        4usize,
        concat!("Alignment of ", stringify!(C__bindgen_ty_1__bindgen_ty_2))
    );
    assert_eq!(
        {
            let struct_instance =
                unsafe { std::mem::zeroed::<C__bindgen_ty_1__bindgen_ty_2>() };
            let struct_ptr =
                &struct_instance as *const C__bindgen_ty_1__bindgen_ty_2;
            let field_ptr = std::ptr::addr_of!(struct_instance.mStepSyntax);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(C__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(mStepSyntax)
        )
    );
    assert_eq!(
        {
            let struct_instance =
                unsafe { std::mem::zeroed::<C__bindgen_ty_1__bindgen_ty_2>() };
            let struct_ptr =
                &struct_instance as *const C__bindgen_ty_1__bindgen_ty_2;
            let field_ptr = std::ptr::addr_of!(struct_instance.mSteps);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(C__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(mSteps)
        )
    );
}
impl Clone for C__bindgen_ty_1__bindgen_ty_2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for C__bindgen_ty_1__bindgen_ty_2 {
    fn default() -> Self {
        unsafe {
            let mut s: Self = ::std::mem::uninitialized();
            ::std::ptr::write_bytes(&mut s, 0, 1);
            s
        }
    }
}
#[test]
fn bindgen_test_layout_C__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<C__bindgen_ty_1>(),
        16usize,
        concat!("Size of: ", stringify!(C__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<C__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(C__bindgen_ty_1))
    );
}
impl Clone for C__bindgen_ty_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq, Eq)]
pub struct C_Segment {
    pub begin: ::std::os::raw::c_int,
    pub end: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_C_Segment() {
    assert_eq!(
        ::std::mem::size_of::<C_Segment>(),
        8usize,
        concat!("Size of: ", stringify!(C_Segment))
    );
    assert_eq!(
        ::std::mem::align_of::<C_Segment>(),
        4usize,
        concat!("Alignment of ", stringify!(C_Segment))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<C_Segment>() };
            let struct_ptr = &struct_instance as *const C_Segment;
            let field_ptr = std::ptr::addr_of!(struct_instance.begin);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(C_Segment),
            "::",
            stringify!(begin)
        )
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<C_Segment>() };
            let struct_ptr = &struct_instance as *const C_Segment;
            let field_ptr = std::ptr::addr_of!(struct_instance.end);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(C_Segment),
            "::",
            stringify!(end)
        )
    );
}
impl Clone for C_Segment {
    fn clone(&self) -> Self {
        *self
    }
}
#[test]
fn bindgen_test_layout_C() {
    assert_eq!(
        ::std::mem::size_of::<C>(),
        20usize,
        concat!("Size of: ", stringify!(C))
    );
    assert_eq!(
        ::std::mem::align_of::<C>(),
        4usize,
        concat!("Alignment of ", stringify!(C))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<C>() };
            let struct_ptr = &struct_instance as *const C;
            let field_ptr = std::ptr::addr_of!(struct_instance.d);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!("Offset of field: ", stringify!(C), "::", stringify!(d))
    );
}
impl Clone for C {
    fn clone(&self) -> Self {
        *self
    }
}
