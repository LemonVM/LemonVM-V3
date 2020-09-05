
// MemoryTags first two digit of u8 tag
const REGVar:u8 = 0b00000000;
const NSVar:u8 = 0b01000000;
const GCVar:u8 = 0b10000000;

// TypeTags last 6 digit of u8 tag
const UndefinedType:u8 = 0x00;
const NullType:u8 = 0x01;
const U8Type:u8 = 0x02;
const I8Type:u8 = 0x03;
const U16Type:u8 = 0x04;
const I16Type:u8 = 0x05;
const U32Type:u8 = 0x06;
const I32Type:u8 = 0x07;
const U64Type:u8 = 0x08;
const I64Type:u8 = 0x09;
const F32Type:u8 = 0x0A;
const F64Type:u8 = 0x0B;

const StringType:u8 = 0x0C;
const SymbolType:u8 = 0x0D;
const VectorType:u8 = 0x0E;
const MapType:u8 = 0x0F;
const ClosureType:u8 = 0x10;
// use to make typed array
const OpaqueType:u8 = 0x11;

// With extension
#[cfg(BIG_INT)]
const BigIntType:u8 = 0x12;
#[cfg(VALUE_TYPE_REFERENCE)]
const REFType:u8 = 0x13;