package main

// 抽象主体
type subject interface {
  register(Observer observer)
  deregister(Observer observer)
  notifyAll()
}
