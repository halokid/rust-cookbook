package main

func main() {
  // todo: 建立主体类
  shirtItem := newItem("Nike Shirt")

  // todo: 建立两个观察者
  observerFirst := &customer{ id: "abc@abc.com" }
  observerSecond := &customer{ id: "xyz@xyz.com" }

  // todo: 给主体类注册观察者
  shirtItem.register(observerFirst)
  shirtItem.register(observerSecond)

  shirtItem.updateAvailability()      // todo: 发布库存可用的通知给观察者
}
