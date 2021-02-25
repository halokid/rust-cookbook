package main

import "log"

/*
具体策略: fifo
 */

type fifo struct {}

func (l *fifo) evict(c *cache) {
  log.Println("Evicting by fifo strategy")
}
