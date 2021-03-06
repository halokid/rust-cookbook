# 观察者模式（Observer pattern）

观察者模式用于建立这样一种依赖关系：当某个对象发生改变的时候，将自动通知其他对象，其他对象将做出相应的反应。发生改变的对象称为观察目标，而被通知的对象称为观察者。一个观察目标可以对应多个观察者，而且这些观察者之间没有相互联系，可以根据需要增加和删除观察者，使得系统更易于扩展。也被称为发布订阅模式（Pub/Sub）、模型-视图（Model/View）模式、源-监听器（Source/Listener）模式或从属者（Dependents）模式。

观察者模式属于行为型模式。

## 模式结构

包含角色：

- 抽象目标（Subject）
- 具体目标
- 抽象观察者（Observer）
- 具体观察者


###  优点

观察者模式的优点

- 观察者模式可以实现表示层和数据逻辑层的分离，并定义了稳定的消息更新传递机制，抽象了更新接口，使得可以有各种各样不同的表示层作为具体观察者角色。
- 观察者模式在观察目标和观察者之间建立一个抽象的耦合。
- 观察者模式支持广播通信。
- 观察者模式符合“开闭原则”的要求。

### 缺点

观察者模式的缺点

- 如果一个观察目标对象有很多直接和间接的观察者的话，将所有的观察者都通知到会花费很多时间。
- 如果在观察者和观察目标之间有循环依赖的话，观察目标会触发它们之间进行循环调用，可能导致系统崩溃。
- 观察者模式没有相应的机制让观察者知道所观察的目标对象是怎么发生变化的，而仅仅只是知道观察目标发生了变化。


### 适用环境

在以下情况下可以使用观察者模式：

- 一个抽象模型有两个方面，其中一个方面依赖于另一个方面。将这些方面封装在独立的对象中使它们可以各自独立地改变和复用。
- 一个对象的改变将导致其他一个或多个对象也发生改变，而不知道具体有多少对象将发生改变，可以降低对象之间的耦合度。
- 一个对象必须通知其他对象，而并不知道这些对象是谁。

需要在系统中创建一个触发链，A对象的行为将影响B对象，B对象的行为将影响C对象……，可以使用观察者模式创建一种链式触发机制。

### 模式应用

观察者模式在软件开发中应用非常广泛，如某电子商务网站可以在执行发送操作后给用户多个发送商品打折信息，某团队战斗游戏中某队友牺牲将给所有成员提示等等，凡是涉及到一对一或者一对多的对象交互场景都可以使用观察者模式。

### 实例说明

利用观察者模式实现事件驱动的日志，用于记录tcp连接事件。

定义Events trait来抽象TCP连接中的事件接口：

- on_connect
- on_error
- on_read
- on_shutdown
- on_pre_read
- on_post_read

定义结构体Logger，为其实现Events，当事件发生的时候，会打印相关信息。

接下来使用HttpClient结构体来执行网络调用，并在事件发生时，通知其注册的hooks（观察者）。

main函数中，使用httpbin.org网站来验证请求并打印数据。


