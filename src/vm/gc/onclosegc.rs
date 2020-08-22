use super::*;
use std::alloc::{Layout, dealloc};

struct OnCloseGC{
    blocks: Vec<OnCloseBlock>,
    pool: Vec<GCInnerValue>
}

struct OnCloseBlock{
    gc: NonNull<OnCloseGC>,
    d: NonNull<GCInnerValue>
}

impl GCBlock for OnCloseBlock{
    fn get_data(&self) -> NonNull<GCInnerValue>{
        self.d
    }
    fn get_data_size(&self) -> usize{
        match unsafe{self.d.as_ref()}{
            GCInnerValue::String(s) => {
                s.len()
            },
            GCInnerValue::Opaque(vec) => {
                vec.len()
            },
            GCInnerValue::Vector(vec) => {
                let mut size = 0;
                for i in vec{
                    if let &Value::GCValue(gcv) = i{
                        size += gcv.get_data_size();
                    }else{
                        size += std::mem::size_of_val(i);
                    }
                }
                size
            },
            GCInnerValue::Map(map) => {
                let mut size = 0;
                for (i,j) in map{
                    if let &Value::GCValue(gcv) = j{
                        size += gcv.get_data_size();
                    }else{
                        size += std::mem::size_of_val(j);
                    }
                    size += i.len();
                }
                size
            },
            _ => unimplemented!()
        }
    }
    fn get_references(&self) -> Vec<NonNull<dyn GCBlock>>{
        match unsafe{self.d.as_ref()}{
            GCInnerValue::String(s) => {
                vec![]
            },
            GCInnerValue::Opaque(vec) => {
                vec![]
            },
            GCInnerValue::Vector(vec) => {
                let mut ret = vec![];
                for i in vec{
                    if let &Value::GCValue(gcv) = i{
                        ret.push(gcv.get_block());
                    }
                }
                ret
            },
            GCInnerValue::Map(map) => {
                let mut ret = vec![];
                for (i,j) in map{
                    if let &Value::GCValue(gcv) = j{
                        ret.push(gcv.get_block());
                    }
                }
                ret
            },
            _ => unimplemented!()
        }
    }

    fn set_value(&mut self, v:Value)-> Value {
        if let Value::GCValue(gcv) = v{
            Value::GCValue(gcv)
        }else{
            v
        }
    }
}

impl GC for OnCloseGC{
    fn on_create(&mut self) {
    }
    fn add_block(&mut self, block:GCInnerValue) -> NonNull<dyn GCBlock> {
        unsafe{
            self.pool.push(block);
            let b = NonNull::new_unchecked(self.pool.last_mut().unwrap());
            let blc = OnCloseBlock{
                gc: NonNull::new_unchecked(self),
                d: b,
            };
            self.blocks.push(blc);
            unsafe{NonNull::new_unchecked(self.blocks.last_mut().unwrap())}
        }
    }
    fn trigger_on_close_function_call(&mut self) {
    }
    fn trigger_on_increse_size(&mut self) {
    }
    fn trigger_on_massive_increse_size(&mut self) {
    }
    fn on_destroy(&mut self) {
    }
}