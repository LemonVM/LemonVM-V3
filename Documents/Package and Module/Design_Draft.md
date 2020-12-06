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
Module在加载之前的字节码状态的时候暂时还没想好如何处理类型信息 `TODO`
原因是我想引入VM自带的Structural Type的同时还想处理Alias问题
但是简单来讲就只是最最基础的类似String或者HashMap之类的字面类型，
当Module在正式加载之后才开始去填充他的实际的类型信息，
同时指引字面类型到某个虚拟机里面的实际类型，这也使得动态加载变得可行。

同时`TODO`我还没有想好动态加载进来的新的Module
对于已经存在的旧的带有Parameter的Module之间的关系，不过我暂时认为

`List<Int>`和`List<Float>`之间不应该存在什么关系（
至少想要他们发生关系的话需要先lift到`Any`类型

所以每一个有parameter构造的module在新的实例加载并使用的时候才会构造新的类型。

## LemonVM Package
Package相当与库或者是可执行文件，Package会有一个配置文件
在这个配置文件里需要描述依赖关系

Package和Module的关系是Module \in Package，通过指定那个Module中那个函数作为main来运行。

### 依赖关系的处理
老生畅谈的问题，比如循环依赖怎么解决

在一个Package被加载的时候，对本体进行缓存后首先去加载他的依赖Package，同时在加载依赖Package的时候
遇到对本体的依赖，当版本信息位于此大版本之内将不会加载，若大版本有区别就报错呗～

同时由于Module的类型信息也是运行时加载的，所以不存在那个类型不匹配之类的。




