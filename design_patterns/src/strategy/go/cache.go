package main

/*
实际策略入口(管理)类
 */

type cache struct {
  storage        map[string]string
  evictionAlgo   evictionAlgo
  capacity       int
  maxCapacity    int
}

func initCache(e evictionAlgo) *cache {
  storage := make(map[string]string)
  return &cache{
    storage:        storage,
    evictionAlgo:   e,
    capacity:       0,
    maxCapacity:    2,
  }
}

func (c *cache) setEvictionAlgo(e evictionAlgo) {
  c.evictionAlgo = e
}

func (c *cache) add(key, value string) {
  if c.capacity == c.maxCapacity {
    c.evict()
  }
  c.capacity++
  c.storage[key] = value
}

func (c *cache) evict() {
  c.evictionAlgo.evict(c)
  c.capacity--
}

func (c *cache) del(key string) {
  delete(c.storage, key)
}






