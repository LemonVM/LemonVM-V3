# Package 文件
TODO
# Module 文件 三个

下面是类C语言伪代码表示的Module二进制文件的存储格式

```c
Value{
    // type tag or info tag
    u8 tag;
    union primitives value;
}

ModuleConstantPool{
    u64 pool_len;
    value pool [pool_len];
}
```

```
ModuleExcutablePool{

}
```

```
ModuleTypeDefs{
    u64 defs_len;
    (name,def) defs[defs_len];
}


fieldmetainfo{

}
FieldInfo{
    // extern public static private ...
    u16 access_mask;
    // field name in constant pool
    u16 name_index;
    u16 type_info_index;

    bool is_function;
    u16 excutable_index;

    hashmap<string,fieldmetainfo> fieldmetainfo;
}


ModuleInfo{
    ...
}
ModuleByteCode {
    u8 signature[5];
    array<fieldinfo> filedinfo;
    hashmap<string,moduleinfo> moduleinfo;
}
```
> 为了平台规范性，module文件存储的所有的数据采用大端存储


## 洞
- debug
- exception table