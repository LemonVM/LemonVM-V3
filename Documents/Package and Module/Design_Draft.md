# LemonVM Module & Package 设计草案
首先就是说为什么存在module
1. 方便去做类似JVM这种class文件一样的模块化动态加载
2. 同时解决模块化的问题，类似JAVA的package的概念

然后就是为什么存在Package文件
1. package文件是用来解决部署的问题的，因为LemonVM的初心在于打造一个
万能的胶水语言的VM，所以为了方便胶水的管理设立一个官方的package
2. 同时去解决依赖问题，这里我觉得JAR和DLL都没有做好，都会陷入JAR地狱和DLL地狱
原因在于一开始没有考虑到现代的这种包管理

## LemonVM Module
Module类似JVM的class文件，包含了头，常量池，调试信息，还有就是改Module的加载方法
### Module 相当于什么
Module大概相当于类型，和这个类型的伴生方法，同时这个类型是如何构造的
因为Module的构造是在LemonVM的加载时期动态完成的，所以也就不存在HKT的问题。

### Module的类型信息
为了满足迅速的加载, 迅速的运行,我决定采用最简单的Nominal Type去限定Module的类型
每个Module都会被标记上TypeInfo,这些TypeInfo可以是泛型的 [详见TypeInfo](../VM%20Default%20Types/Type.md),
默认在打包的过程中,会生成动态调用的代码和静态调用的代码,
分别使用invokevirtual之类的和invokedynamic之类的,如果TypeInfo里面还有Hole
那么该Module的所有涉及到泛型参数的调用将会是invokedynamic

## LemonVM Package
Package相当与库或者是可执行文件，Package会有一个配置文件
在这个配置文件里需要描述依赖关系

Package和Module的关系是Module \in Package，通过指定那个Module中那个函数作为main来运行。

### Package和Module的区别

- 所有的Package都需要在内部处理依赖问题,不可以向外部传递依赖关系
- Package需要解决的是一个大的问题,比如一个框架,而Module解决的是一个小的问题
- Package可以依赖于Package和Module,Module尽可以依赖于Module
- 建议如果对一个大型框架添加功能请重新打包,而不是使用动态装载器

### 依赖关系的处理
老生畅谈的问题，比如循环依赖怎么解决

在一个Package被加载的时候，对本体进行缓存后首先去加载他的依赖Package，同时在加载依赖Package的时候
遇到对本体的依赖，当版本信息位于此大版本之内将不会加载，若大版本有区别就报错呗～

同时由于Module的类型信息也是运行时加载的，所以不存在那个类型不匹配之类的。




