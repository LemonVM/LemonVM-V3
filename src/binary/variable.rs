use super::TypeTags;
use std::collections::BTreeMap;
// strong type extension
enum Type{
    Mono(TypeTags),
    Record(BTreeMap<String,TypeTags>),
    // last in function is the return value type
    Function(Vec<Type>)
}

struct Variable{
    name:u16,
    access: u8,
    // strong type extension
    type_info: Type,
}