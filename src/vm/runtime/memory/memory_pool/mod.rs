pub static LemonVMMemoryPool: *mut u8 = std::ptr::null_mut();
pub static LemonVMRoots: *mut u8 = std::ptr::null_mut();
struct Pool{

}

union Ref{
    primitive: u64,
    module: RuntimeModule,
}

struct GCRef{
}