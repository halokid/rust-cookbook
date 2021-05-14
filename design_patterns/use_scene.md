各种设计模式使用的应用场景收集精要
=========================================

为什么是收集精要？不废话，直接就直白场景总结，汇聚本人了解过的使用场景之精要。


### 工厂模式
- 创建一个产品类实例的时候， 什么叫产品类？这个要看具体的业务需求了，举几个例子， 比如订单类、商品类等这些都属于
- 有一个要点就是这个产品实例类不包含其他类的创建过程
关键代码：创建过程在其子类执行。

### 单例模式
- 当某一个内存句柄只需要创建一次， 多个代码逻辑共用这个句柄的时候， 比如数据库连接等
关键代码：构造函数是私有的。

### 建造者模式 
- 当一个类的构造函数有多个参数， 并且参数有些是必须，有些是可选，参数要看具体的逻辑情况而定的情况下，考虑使用构造者模式。
- 当实体构建者类 包含 目标类， 或者是目标类子类的时候
- 抽象构建类方法不应该有任何参数， 方法里面具体的逻辑才调用目标类的方法、定义目标类的方法参数
- 针对实体构建者类的实际赋值过程逻辑通常定义在目标类, 构建者类通常只有构造、new等方法
关键代码：建造者：创建和提供实例，导演：管理建造出来的实例的依赖关系。


### 原型模式
- 重复生成类实例的优化， 每次new一个类时，初始化会需要比较多的资源， 直接用原型模型clone类实例，科优化性能
- 多个类实例有重复的属性、方法， 然后在各种逻辑场景中有可能会修改类实例的数据时
关键代码： 1、实现克隆操作，在 JAVA 继承 Cloneable，重写 clone()，在 .NET 中可以使用 Object 类的 MemberwiseClone() 方法来实现对象的浅拷贝或通过序列化的方式来实现深拷贝。 2、原型模式同样用于隔离类对象的使用者和具体类型（易变类）之间的耦合关系，它同样要求这些"易变类"拥有稳定的接口

### 配适器模式 
- 假如有多个不兼容的接口，想统一把这些不兼容的接口封装成统一的出处，也就是搭建一个桥梁，通过这个桥梁可以通往这些不兼容的接口，这个桥梁就是配适器
关键代码：适配器继承或依赖已有的对象，实现想要的目标接口。

### 桥接模式
- 当要把抽象的部分跟实现的部分分离的场景， 因为实现就是单纯的实现， 不管抽象，抽象的话，可以按照具体的业务需求去更改抽象，而不会影响实现
关键代码：抽象类依赖实现类。

### 过滤器模式
- 当需要使用不同的标准来过滤一组对象、数据等，并且需要通过逻辑运算以解耦的方式把它们连接起来。
- 常见的按照某个条件filter组合数据场景


### 组合模式
- 把一组相似的对象当作一个单一的对象。组合模式依据树形结构来组合对象。
- 通常用于树形、链表、多节点组合等数据结构的场景
关键代码：树枝内部组合该接口，并且含有内部属性 List，里面放 Component。

### 装饰器模式
- 允许向一个现有的对象添加新的功能，同时又不改变其结构, 从逻辑上是作为一个现有类的封装
- 常见的插件式设计就是这种模式， 就是增加一个功能就是多一个plugin，不改变原来的源码结构
- 我们为了而给一个类增加功能, 通常都采用增加子类的形式， 但是功能越来越多，子类会很膨胀，很难管理，在不想增加子类的情况下，扩展原有类的功能也可以采用这种模式
关键代码： 1、Component 类充当抽象角色，不应该具体实现。 2、修饰类引用和继承 Component 类，具体扩展类重写父类方法。

### 外观模式
- 隐藏系统的复杂性，并向客户端提供了一个客户端可以访问系统的接口
- 为了尽量用对接方简单调用而设计的一种模式, 提供了客户端请求的简化方法和对现有系统类方法的委托调用。
- 这种模式设计的源码结构，可以比较清晰的分析到源码的思路，假如是自己写的代码，日后再分析源码结构时，可以快速了解架构设计的思路
关键代码：在客户端和复杂系统之间再加一层，这一层将调用顺序、依赖关系等处理好。


### 享元模式
- 主要用于减少创建对象的数量，以减少内存占用和提高性能
- 业务场景符合可以重用现有的同类对象
关键代码：用 HashMap 存储这些对象。

### 代理模式
- 一个类代表另一个类的功能, 为其他对象提供一种代理以控制对这个对象的访问。
- 使用场景通常都是 想在访问一个类时做一些控制，然后就搞一个代理类出来代理实际要访问的类
- 这个代理类又叫中间层、代理人等等， 通常是 middleware等单词
- 通过调用proxy类的方法去调用真正的处理逻辑
关键代码：实现与被代理类组合。

### 责任链模式
- 简单来说就是建立一条数据处理链， 处理接收者1搞不定， 就传给接收者2， 再搞不定就传给接收者3， 以此类推
- 数据结构的形式是建立一条处理链， 是链的形式
- 避免请求发送者与接收者耦合在一起
- 责任链处理的接口 跟 实际处理逻辑的实体类 源码是分开的
关键代码：Handler 里面聚合它自己，在 HandlerRequest 里判断是否合适，如果没达到条件则向下传递，向谁传递之前 set 进去。

### 命令模式
- 一种数据驱动的设计模式，它属于行为型模式。请求以命令的形式包裹在对象中，并传给调用对象。调用对象寻找可以处理该命令的合适的对象，并把该命令传给相应的对象，该对象执行命令
- 将"行为请求者"与"行为实现者"解耦
关键代码: 定义三个角色：1、received 真正的命令执行对象 2、Command 3、invoker 使用命令对象的入口

### 解释器模式
- 构建环境类，包含解释器之外的一些全局信息，一般是 HashMap
- 主要用来评估语言的语法或表达式的作用

### 迭代器模式
- 用于顺序访问集合对象的元素，不需要知道集合对象的底层表示
关键代码：定义接口：hasNext, next。

### 中介者模式
- 是用来降低多个对象和类之间的通信复杂性。这种模式提供了一个中介类，该类通常处理不同类之间的通信，并支持松耦合，使代码易于维护。
何时使用：多个类相互耦合，形成了网状结构
关键代码：对象 Colleague 之间的通信封装到一个类中单独处理。

### 备忘录模式 
- 保存一个对象的某个状态，以便在适当的时候恢复对象
- 意图：在不破坏封装性的前提下，捕获一个对象的内部状态，并在该对象之外保存这个状态。
- 主要解决：所谓备忘录模式就是在不破坏封装的前提下，捕获一个对象的内部状态，并在该对象之外保存这个状态，这样可以在以后将对象恢复到原先保存的状态。
- 何时使用：很多时候我们总是需要记录一个对象的内部状态，这样做的目的就是为了允许用户取消不确定或者错误的操作，能够恢复到他原先的状态，使得他有"后悔药"可吃。
- 如何解决：通过一个备忘录类专门存储对象状态。
- 关键代码：客户不与备忘录类耦合，与备忘录管理类耦合。

### 观察者模式（更具体看代码里面的readme）
- 当对象间存在一对多关系时，则使用观察者模式（Observer Pattern）。比如，当一个对象被修改时，则会自动通知依赖它的对象
- 意图：定义对象间的一种一对多的依赖关系，当一个对象的状态发生改变时，所有依赖于它的对象都得到通知并被自动更新。
- 主要解决：一个对象状态改变给其他对象通知的问题，而且要考虑到易用和低耦合，保证高度的协作。
- 何时使用：一个对象（目标对象）的状态发生改变，所有的依赖对象（观察者对象）都将得到通知，进行广播通知。
- 如何解决：使用面向对象技术，可以将这种依赖关系弱化。
- 关键代码：在抽象类里有一个 ArrayList 存放观察者们。


### 状态模式
- 类的行为是基于它的状态改变的。这种类型的设计模式属于行为型模式。
- 意图：允许对象在内部状态发生改变时改变它的行为，对象看起来好像修改了它的类。
- 主要解决：对象的行为依赖于它的状态（属性），并且可以根据它的状态改变而改变它的相关行为。
- 何时使用：代码中包含大量与对象状态有关的条件语句。
- 如何解决：将各种具体的状态类抽象出来。
- 关键代码：通常命令模式的接口中只有一个方法。而状态模式的接口中有一个或者多个方法。而且，状态模式的实现类的方法，一般返回值，或者是改变实例变量的值。也就是说，状态模式一般和对象的状态有关。实现类的方法有不同的功能，覆盖接口中的方法。状态模式和命令模式一样，也可以用于消除 if...else 等条件选择语句。

### 空对象模式
- 在空对象模式（Null Object Pattern）中，一个空对象取代 NULL 对象实例的检查。Null 对象不是检查空值，而是反应一个不做任何动作的关系。这样的 Null 对象也可以在数据不可用的时候提供默认的行为。
- 在空对象模式中，我们创建一个指定各种要执行的操作的抽象类和扩展该类的实体类，还创建一个未对该类做任何实现的空对象类，该空对象类将无缝地使用在需要检查空值的地方a

> 我们将创建一个定义操作（在这里，是客户的名称）的 AbstractCustomer 抽象类，和扩展了 AbstractCustomer 类的实体类。工厂类 CustomerFactory 基于客户传递的名字来返回 RealCustomer 或 NullCustomer 对象。 
  NullPatternDemo，我们的演示类使用 CustomerFactory 来演示空对象模式的用法。


### 策略模式
- 在策略模式（Strategy Pattern）中，一个类的行为或其算法可以在运行时更改。这种类型的设计模式属于行为型模式。
- 在策略模式中，我们创建表示各种策略的对象和一个行为随着策略对象改变而改变的 context 对象。策略对象改变 context 对象的执行算法。


### 模板模式
- 一个抽象类公开定义了执行它的方法的方式/模板。它的子类可以按需要重写方法实现，但调用将以抽象类中定义的方式进行。
- 意图：定义一个操作中的算法的骨架，而将一些步骤延迟到子类中。模板方法使得子类可以不改变一个算法的结构即可重定义该算法的某些特定步骤。
- 主要解决：一些方法通用，却在每一个子类都重新写了这一方法。
- 何时使用：有一些通用的方法。
- 如何解决：将这些通用算法抽象出来。
- 关键代码：在抽象类实现，其他步骤在子类实现。

### 访问者模式
- 使用了一个访问者类，它改变了元素类的执行算法。通过这种方式，元素的执行算法可以随着访问者改变而改变
- 意图：主要将数据结构与数据操作分离。
- 主要解决：稳定的数据结构和易变的操作耦合问题。
- 何时使用：需要对一个对象结构中的对象进行很多不同的并且不相关的操作，而需要避免让这些操作"污染"这些对象的类，使用访问者模式将这些封装到类中。
- 如何解决：在被访问的类里面加一个对外提供接待访问者的接口。
- 关键代码：在数据基础类里面有一个方法接受访问者，将自身引用传入访问者。


### 参考
https://www.runoob.com/design-pattern/composite-pattern.html







