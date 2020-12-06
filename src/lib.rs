pub mod binary;
pub mod vm;

pub fn lemonvm_init() -> *mut u8{
    // load bytecode
    // init runtime memories
}

pub fn lemonvm_run(lemonvm_instance:*mut u8,function_name: std::ffi::CString){
    // look for function to run
}

pub fn lemonvm_destroy(lemonvm_instance:*mut u8){
}

pub fn lemonvm_start(){
    // init
    // run main function

    // destroy
}