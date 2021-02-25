package main

import "log"

// 具体观察者类

type customer struct {
  id  string
}

// todo: 观察者接收到通知之后， 具体执行的行为
func (c *customer) update(itemName string) {
  log.Printf("Sending email to customer %s for item %s\n", c.id, itemName)
}

func (c *customer) getID() string {
  return c.id
}


