pub mod constant;
pub mod bytecode;

pub enum TypeTags{
    // Primitives
        Undefined = 0,
        Null,
        Boolean,
        Number,
        String,
        Symbol,
    // Object
        Object,
    // use to make typed array
        Opaque,
    // With extension
        #[cfg(BIG_INT)]
        BigInt,
        #[cfg(VALUE_TYPE_REFERENCE)]
        REF
    }