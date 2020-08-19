// GC are splited in three generations
// young old and immortal
// young generation uses MS gc algorithm
// old uses three color concurrent gc algorithm with write barrier
// immortal also uses MS but normally wouldn't active

// GC Trigger
// young GC will be triggered after very short period                        default every function call
// old GC will be triggerd when memory increase certain value                default 2m
// immutable GC will be triggered when memory increate to a certain value    default 20m

// Downgrade
// for r in root
//     if r is young
//         mark young
// for r in root
//     if r in old
//         color old

use crate::config::*;
struct YoungGCObject {
    object_ptr: *mut u8,
    // exclude the references
    object_size: usize,
    // every type of GCObject
    // if it hold ref of an old obj then directly color it to gray
    // if it hold ref og an imm obj then mark it
    references: Vec<*mut GCObject>,

    // MS mark, to clean !marked  object
    marked: bool,
    // if age > GC_OLD_THRESHOLD -> OldGCObject
    age: u8,
}

enum TriColor {
    White,
    Gray,
    Black,
}
enum WB {
    None,
    Set,
}

struct OldGCObject {
    object_ptr: *mut u8,
    object_size: usize,
    // could not hold an YoungGCObject reference
    // so once it try to hold the YoungGCObjectReference upgrade it and change color to gray,
    references: Vec<*mut GCObject>,

    color: TriColor,
    wb: WB,
}

struct ImmortalObject {
    object_ptr: *mut u8,
    object_size: usize,
    // could not hold any reference except the reference is Immortal
    references: Vec<*mut GCObject>,

    // MS mark, to clean !marked  object
    marked: bool,
}
enum GCObject {
    YoungGCObject(YoungGCObject),
    OldGCObject(OldGCObject),
    ImmortalObject(ImmortalObject),
}

// enum Generation{
//     Young,
//     Old,
//     Immortal
// }

impl GCObject {
    // both mark and unmark
    fn mark_young(&mut self, path: &mut Vec<*mut Self>) {
        use GCObject::*;
        path.push(self);
        match self {
            YoungGCObject(young) => {
                young.marked = true;
                young.age += 1;
                for r in &mut young.references {
                    if !path.contains(r) {
                        unsafe {
                            (&mut **r).mark_young(path);
                        }
                    }
                }
            }
            OldGCObject(old) => {
                old.color = TriColor::Black;
                old.wb = WB::Set;
            }
            ImmortalObject(imm) => {
                imm.marked = true;
            }
        }
    }
    // coloring when color black add write barrier
    fn color_old(&mut self, color: TriColor, path: &mut Vec<*mut Self>) {
        use GCObject::*;
        path.push(self);
        match self {
            YoungGCObject(young) => panic!("ERROR! old generation object holds an young reference"),
            OldGCObject(old) => match color {
                TriColor::White => {
                    old.color = color;
                    old.wb = WB::None;
                }
                TriColor::Gray => {
                    old.color = color;
                    old.wb = WB::None;
                    for r in &mut old.references {
                        if !path.contains(r) {
                            unsafe {
                                (&mut **r).color_old(TriColor::White, path);
                            }
                        }
                    }
                }
                TriColor::Black => {
                    old.color = color;
                    old.wb = WB::Set;
                    for r in &mut old.references {
                        if !path.contains(r) {
                            unsafe {
                                (&mut **r).color_old(TriColor::Gray, path);
                            }
                        }
                    }
                }
            },
            ImmortalObject(imm) => {
                imm.marked = true;
            }
        }
    }
    // both mark and unmark
    fn mark_imm(&mut self, path: &mut Vec<*mut Self>) {
        use GCObject::*;
        path.push(self);
        match self {
            ImmortalObject(imm) => {
                imm.marked = true;
                for r in &mut imm.references {
                    if !path.contains(r) {
                        unsafe {
                            (&mut **r).mark_imm(path);
                        }
                    }
                }
            }
            _ => panic!(
                "ERROR! immortal generation object could only hold immortal generation reference"
            ),
        }
    }

    fn young_to_old(&mut self, path: &mut Vec<*mut Self>) {
        // use GCObject::*;
        match self {
            GCObject::YoungGCObject(young) => {
                let mut new_obj = OldGCObject {
                    object_ptr: young.object_ptr,
                    object_size: young.object_size,
                    color: TriColor::Gray,
                    wb: WB::None,
                    references: young.references.clone(),
                };
                for r in &mut new_obj.references {
                    if !path.contains(&r) {
                        unsafe {
                            (&mut **r).young_to_old(path);
                        }
                    }
                }
                *self = GCObject::OldGCObject(new_obj);
            }
            _ => {}
        }
    }
}

struct GCPool {
    young_generation_pool: Vec<*mut GCObject>,
    old_generation_pool: Vec<*mut GCObject>,
    immortal_generation_pool: Vec<*mut GCObject>,
}

impl GCPool {
    fn new() -> Self {
        GCPool {
            young_generation_pool: vec![],
            old_generation_pool: vec![],
            immortal_generation_pool: vec![],
        }
    }
    // usually use this
    fn add_young(&mut self, ptr: *mut u8, size: usize) -> *mut GCObject {
        let young_gc_obj = Box::new(GCObject::YoungGCObject(YoungGCObject {
            object_ptr: ptr,
            object_size: size,
            marked: false,
            age: 0,
            references: vec![],
        }));
        let young_gc_obj_ptr = Box::leak(young_gc_obj);
        self.young_generation_pool.push(young_gc_obj_ptr);
        return young_gc_obj_ptr;
    }
    // normally should not use
    fn add_old(&mut self, ptr: *mut u8, size: usize) -> *mut GCObject {
        let old_gc_obj = Box::new(GCObject::OldGCObject(OldGCObject {
            object_ptr: ptr,
            object_size: size,
            color: TriColor::Gray,
            wb: WB::None,
            references: vec![],
        }));
        let old_gc_obj_ptr = Box::leak(old_gc_obj);
        self.old_generation_pool.push(old_gc_obj_ptr);
        return old_gc_obj_ptr;
    }
    // used in global objects
    fn add_imm(&mut self, ptr: *mut u8, size: usize) -> *mut GCObject {
        let imm_gc_obj = Box::new(GCObject::ImmortalObject(ImmortalObject {
            object_ptr: ptr,
            object_size: size,
            marked: false,
            references: vec![],
        }));
        let imm_gc_obj_ptr = Box::leak(imm_gc_obj);
        self.immortal_generation_pool.push(imm_gc_obj_ptr);
        return imm_gc_obj_ptr;
    }
    fn upgrade_young(&mut self) {
        for r in &mut self.young_generation_pool {
            unsafe {
                let obj = &mut **r;
                if let GCObject::YoungGCObject(young) = obj {
                    if young.age >= YOUNG_GENERATION_UPGRADE_AGE as u8 {
                        let mut path = vec![];
                        obj.young_to_old(&mut path);
                        // TODO: now lots of obj is old but storage in young pool
                        // we need to add them all into old pool
                    }
                }
            }
        }
    }
    fn young_generation_gc(&mut self, root: &mut Vec<*mut GCObject>) {
        mark_young(root);
        let mut i = 0;
        // clean garbage
        loop {
            if self.young_generation_pool.len() == i {
                break;
            } else {
                unsafe {
                    let obj = &*self.young_generation_pool[i];
                    use GCObject::*;
                    match obj {
                        YoungGCObject(young) => {
                            if !young.marked {
                                self.young_generation_pool.remove(i);
                                continue;
                            }
                        }
                        _ => panic!("ERROR! other type in young gen pool"),
                    }
                }
            }
        }
        // upgrade
        self.upgrade_young();
        // move old to old pool
        i = 0;
        loop {
            if self.young_generation_pool.len() == i {
                break;
            } else {
                unsafe {
                    let obj = &*self.young_generation_pool[i];
                    use GCObject::*;
                    match obj {
                        OldGCObject(old) => {
                            self.old_generation_pool.push(self.young_generation_pool[i]);
                            self.young_generation_pool.remove(i);
                            continue;
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    fn color_all_old(&mut self, root: &mut Vec<*mut GCObject>, root_colored: bool) -> bool {
        if !root_colored {
            color_old(root);
        }
        let exist_gay = false;
        if !exist_gay {
            return true;
        } else {
            // color all gay
            return false;
        }
    }
    fn old_generation_gc(&mut self) {
        // delete all white
        // upgrade nodes
        // all color -1
    }
}

fn mark_young(root: &mut Vec<*mut GCObject>) {
    root.iter_mut().for_each(|f| unsafe {
        let obj = &mut **f;
        let mut path = vec![];
        obj.mark_young(&mut path);
    });
}

fn color_old(root: &mut Vec<*mut GCObject>) {
    root.iter_mut().for_each(|f| unsafe {
        let obj = &mut **f;
        let mut path = vec![];
        obj.color_old(TriColor::Black, &mut path);
    });
}

fn mark_imm(root: &mut Vec<*mut GCObject>) {
    root.iter_mut().for_each(|f| unsafe {
        let obj = &mut **f;
        let mut path = vec![];
        obj.mark_imm(&mut path);
    });
}
