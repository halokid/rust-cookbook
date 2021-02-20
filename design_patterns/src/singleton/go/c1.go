package main

import (
  "log"
  "sync"
)

type singleton struct {}

var instance *singleton
var once sync.Once

func Getinstance() *singleton {
  once.Do(func() {
    log.Print("----- Only create instance Once -----")
    instance = &singleton{}
  })
  return instance
}

func main() {
  wg := sync.WaitGroup{}
  wg.Add(30)
  for i := 0; i < 30; i++ {
    inst := Getinstance()
    log.Printf("inst ----------- %+v", inst)
    wg.Done()
  }
  wg.Wait()
}


