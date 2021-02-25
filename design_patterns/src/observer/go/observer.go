package main

// 抽象观察者

type observer interface {
  update(string)
  getID() string
}

