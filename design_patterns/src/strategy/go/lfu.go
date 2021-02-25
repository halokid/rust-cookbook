package main

import "log"

/*
具体策略: lfu
 */

type lfu struct {}

func (l *lfu) evict(c *cache) {
  log.Println("Evicting by lfu strategy")
}
