package main

import (
  "log"
  "strings"
)
/*
工厂方法模式
 */

type Product interface {
  convert(s string)   string
}

type Factory interface {
  createProduct()   Product
  convert(s string)   string
}

// 函数是第一公民的原则，把convert的逻辑提取出来封装
// 因为interface不能定义逻辑
func convertWrap(p Product, s string) string {
  log.Printf("convertWrap: %s", s)
  return p.convert(s)
}

// ---------------------------------------
type ProductX struct {}

func (p ProductX) convert(s string) string {
  log.Printf("ProductX convert: %s", strings.ToUpper(s))
  return strings.ToUpper(s)
}

func newProductX() Product {    // 映射出ProductX的Product特性
  return ProductX{}
}


// --------------------------------------
type FacotryAll struct {}

func (f FacotryAll) convert(s string) string {
  return convertWrap(f.createProduct(), s)
}

func (f FacotryAll) createProduct() Product {
  return newProductX()
}

func NewFactoryAll() Factory {
  return FacotryAll{}
}

func main() {
  f := NewFactoryAll()
  log.Printf("%s",f.convert("HaloKid"))
}


