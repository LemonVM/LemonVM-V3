# 内存布局

## 线程私有的信息
```c
ThreadInfo{
    u64 pc;
    u8 args_len;
    Value call_args[args_len];
    Value registers[MAX_REGISTER_LEN];
}
```

## 全局共享的信息
只能加不能减，不能改的
- Global Values
- Runtime Packages
    - Runtime Modules
        - Runtime Constant Pool

GC帮着管理的
- Reference Typed Values
