package main

import "log"

/*
原型模式
 */

type Prototype interface {
  setX()
  ssetY()
  Clone() *Object
}

type Object struct {
  x     uint8
  y     uint8
}

func (o *Object) setX() {
  panic("implement me")
}

func (o *Object) ssetY() {
  panic("implement me")
}

func (o *Object) Clone() *Object {
  oc := *o
  return &oc      // 浅拷贝
}

func NewObject() Prototype {    // todo: 相当于在 rust里面 impl trait for struct
  return &Object{
    x: 100,
    y: 200,
  }
}

func main() {
  origin := NewObject()
  log.Printf("origin --------------- %+v, %+v", &origin, origin)
  objc := origin.Clone()
  log.Printf("objc --------------- %+v, %+v", &objc, objc)
}





