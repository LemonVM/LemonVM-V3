use std::collections::BTreeMap;

pub struct Reader {
    data: *const u8,
    pos: usize,
}

impl Reader {
    pub fn new(data: *const u8) -> Reader {
        Reader { data, pos: 0 }
    }

    pub fn read_u8(&mut self) -> u8 {
        unsafe {
            let b = *self.data.add(self.pos);
            self.pos += 1;
            b
        }
    }
    pub fn read_u16(&mut self) -> u16 {
        unsafe {
            let b = *(self.data.add(self.pos) as *const u16);
            self.pos += 2;
            b
        }
    }
    pub fn read_u32(&mut self) -> u32 {
        unsafe {
            let b = *(self.data.add(self.pos) as *const u32);
            self.pos += 4;
            b
        }
    }
    pub fn read_u64(&mut self) -> u64 {
        unsafe {
            let b = *(self.data.add(self.pos) as *const u64);
            self.pos += 8;
            b
        }
    }
    pub fn read_vec<T, F>(&mut self, f: F) -> Vec<T>
    where
        F: Fn(&mut Reader) -> T,
    {
        let n = self.read_u16() as usize;
        let mut vec = Vec::with_capacity(n);
        for _i in 0..n {
            vec.push(f(self));
        }
        vec
    }
    pub fn read_map<K , V , F>(&mut self, f: F) -> BTreeMap<K,V>
    where
        F: Fn(&mut Reader) -> (K,V),
        K: Ord,
    {
        let n = self.read_u16() as usize;
        let mut map = BTreeMap::new();
        for _i in 0..n {
            let (k,v) = f(self);
            map.insert(k,v);
        }
        map
    }
}