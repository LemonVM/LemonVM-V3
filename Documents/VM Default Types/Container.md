# Containers


## Array
```
struct Array<T>{
    int len
    T*len elems
}
```
二进制储存 仅当 T 是基础类型的时候
ARRAYTYPE TTYPE TLEN LEN ELEMS

## HashMap
二进制储存 仅当 T F 是基础类型的时候
HASHMAPTYPE TTYPE FTYPE LEN (ELEMT ELEMF)+

## String
### ByteString

ByteString 是一个可变长度的U8动态数组
使用ConstString加载进来
里面有encoding属性
支持转换到String