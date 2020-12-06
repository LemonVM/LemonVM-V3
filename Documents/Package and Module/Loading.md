# Loading
首先, VM作为主运行时而非寄生运行时的时候, 使用类似JVM的方式进行目录查找
- Bootstrap : `~/.lemonvm/bootstrap/*`
- Extension : `~/.lemonvm/extension/*`
- User : `. | LEMONVM_MODULE_PATH | -mp "string"`

也就这仨, 具体有三种Entry
- Dir + Wildcard
- Package : zip file with manifest.json
- String (也就是裸指针)
