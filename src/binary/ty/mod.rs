use std::collections::HashMap;

use containers::Array;

pub mod containers;
pub mod tags;

struct Type{
    name: String,
    typeinfo: TypeInfo
}

struct TypeInfo{
    is_hole: bool,
    is_function: bool,
    
    is_poly: bool,
    poly_data: PolyData,
}

struct PolyData {
    holes_count: u16,
    type_parameters: HashMap<String,Type>
}