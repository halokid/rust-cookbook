package main

import "log"

/*
具体策略: lru
 */

type lru struct {}

func (l *lru) evict(c *cache) {
  log.Println("Evicting by lru strategy")
}
