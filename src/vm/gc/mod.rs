// a good gc that targetting a big project
pub mod lemongengc;
// a dum gc for a very small script no need gc pause
pub mod onclosegc;

pub trait GCObject{
    fn get_data_ptr(&self)->*mut u8;
    fn get_data_size(&self)->*mut u8;
    fn get_references(&self)->Vec<*mut dyn GCObject>;
    
    fn add_reference(&mut self, ref_: *mut dyn GCObject);
}

pub trait GC{
    // young generation
    fn trigger_on_close_function_call(&mut self);
    // old generation
    fn trigger_on_increse_size_range(&mut self);
    // imm generation
    fn trigger_on_massive_increse_size_range(&mut self);
}