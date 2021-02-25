package main

/*
策略抽象接口
 */

type evictionAlgo interface {
  evict(c *cache)
}

