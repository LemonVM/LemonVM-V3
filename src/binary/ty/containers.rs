#[repr(C)]
pub struct Array {
    len: u64,
    elem_ty: u8,
    elem: *mut u8,
}

#[repr(C)]
struct Map{
    len: u64,
    key_ty: u8,
    elem_ty: u8,
    pair: *mut u8,
}

#[repr(C)]
struct ByteString{
    encoding: Array,
    data: Array,
}
