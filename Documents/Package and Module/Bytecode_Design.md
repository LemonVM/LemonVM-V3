# Package 文件
TODO
# Module 文件

下面是类C语言伪代码表示的Module二进制文件的存储格式
```c
ModuleByteCode {
    u8 signature[5];
    u32 version;
    u8 extension_length;
    u8 extensions_id [extension_length];
    // constant pool
    u64 pool_len;
    value pool [pool_len];
    // fields
    u32 field_len;
    FieldInfo fields [field_len];
    // infos such as debug info
    u8 info_len;
    Value infos [info_len];
}
```
> 为了平台规范性，module文件存储的所有的数据采用大端存储

这是module中每一field的结构定义
```c
FieldInfo{
    // extern public static private ...
    u16 access_mask;
    // field name in constant pool
    u16 name_index;
    u16 type_info_index;
    u8 info_len;
    Value infos [info_len];
    // for stack allocate data
    u32 value_index;
}
```

这是Constant Pool中任意一个Constant的定义
```c
Value{
    // type tag or info tag
    u8 tag;
    union primitives value;
}
```