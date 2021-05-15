#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub type mbedtls_mpi_uint = ::std::os::raw::c_uint;
/// \brief          MPI structure
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mbedtls_mpi {
    ///<  integer sign
    pub s: ::std::os::raw::c_int,
    ///<  total # of limbs
    pub n: ::std::os::raw::c_ulong,
    ///<  pointer to limbs
    pub p: *mut mbedtls_mpi_uint,
}
#[test]
fn bindgen_test_layout_mbedtls_mpi() {
    assert_eq!(
        ::std::mem::size_of::<mbedtls_mpi>(),
        24usize,
        concat!("Size of: ", stringify!(mbedtls_mpi))
    );
    assert_eq!(
        ::std::mem::align_of::<mbedtls_mpi>(),
        8usize,
        concat!("Alignment of ", stringify!(mbedtls_mpi))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<mbedtls_mpi>() };
            let struct_ptr = &struct_instance as *const mbedtls_mpi;
            let field_ptr = std::ptr::addr_of!(struct_instance.s);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(mbedtls_mpi),
            "::",
            stringify!(s)
        )
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<mbedtls_mpi>() };
            let struct_ptr = &struct_instance as *const mbedtls_mpi;
            let field_ptr = std::ptr::addr_of!(struct_instance.n);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(mbedtls_mpi),
            "::",
            stringify!(n)
        )
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<mbedtls_mpi>() };
            let struct_ptr = &struct_instance as *const mbedtls_mpi;
            let field_ptr = std::ptr::addr_of!(struct_instance.p);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(mbedtls_mpi),
            "::",
            stringify!(p)
        )
    );
}
impl Default for mbedtls_mpi {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
