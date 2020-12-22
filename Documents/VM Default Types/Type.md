# Type

## Primitive Type
```
bool (u|i)[8-64] could not pass as generics type parameter
Bool (U|I)[8-64] boxed primitive 快速开箱, 默认拷贝
```

## TypeInfo
用于Reflection
```
struct Type{
    name: String
    typeinfo: Hole | TypeInfo
}


struct TypeInfo{
    is_function: Boolean

    is_reference: Boolean

    poly: Option<
        holes_count: U8
        type_parameters: HashMap<String,Type>
    >
}
```

## Hole Type
用于充当泛型中的Type Parameter, 同时单独出现的时候可以当Any使用
也就是说一个泛型如果没有补全Type Parameter那么里面的Hole的开销就是Any的开销
补全了Type Parameter之后可以将这部分开销抹平