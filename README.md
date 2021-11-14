# Factory method模式

工厂方法模式（Factory Method Pattern）在创建对象时不会对客户端暴露创建逻辑，通过使用一个共同的接口来指向新创建的对象。在 Rust 中实现工厂方法模式，主要通过`trait`和`enum`来实现。

# 实战

假设我们有一个物流中心（Logistics），它能把货物运送出去（Deliver），其中包含了路运和海运。物流中心每发出一个货运实例，就要记录发出的货运量（Payload）。

![1](https://refactoring.guru/images/patterns/content/factory-method/factory-method-en.png?id=cfa26f33dc8473e803fa)

我们定义一个枚举类型来表明我们要创建的货运类型，然后在`main`函数中，我们只需要通过「物流工厂」来创建一个货运实例，而不用分别为路运和海运创建对象的实例，它们也能完成送货的行为。

运行后效果如下：

```bash
Deliver by sea in a container.
Deliver by land in a box.
Deliver by land in a box.
```



# 参考资料

- [工厂模式-菜鸟教程](https://www.runoob.com/design-pattern/factory-pattern.html)
- [原子之音的视频](https://www.bilibili.com/video/BV1oo4y117rv?p=2&spm_id_from=pageDriver)
- [Factory Method](https://refactoring.guru/design-patterns/factory-method)
- [Factory design Pattern in Rust](https://chercher.tech/rust/factory-design-pattern-rust#)

---
