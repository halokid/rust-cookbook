package main

import "sync"

type singleton struct {}

var instance *singleton
var once sync.Once

func Getinstance() *singleton {
  once.Do(func() {
    instance = &singleton{}
  })
  return instance
}



