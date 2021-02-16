package main

/*
抽象工厂模式
*/

type ProductXX interface {
  getValue()  string
}

type ProductYY interface {
  getValue()  string
}

type AbstractFactory interface {
  createProductXX()   ProductXX
  createProductYY()   ProductYY
}

//-----------------------------------------------------
type ProductXXs struct {}

func (p ProductXXs) getValue() string {
  return "ProductXXs value"
}

func NewProductXXs() ProductXX {
  return ProductXXs{}
}

// ---------------------------------------------------
type ProductYYs struct {}

func (p ProductYYs) getValue() string {
  return "ProductYYs value"
}

func NewProductYYs() ProductYY {
  return ProductYYs{}
}



// -------------------------------------------------
type FactoryA struct {}

func (f FactoryA) createProductXX() ProductXX {
  return ProductXXs{}
}

func (f FactoryA) createProductYY() ProductYY {
  return ProductYYs{}
}

func NewFactoryA() AbstractFactory {
  return FactoryA{}
}


// ------------------------------------------------
type FactoryB struct {}

func (f FactoryB) createProductXX() ProductXX {
  return ProductXXs{}
}

func (f FactoryB) createProductYY() ProductYY {
  return ProductYYs{}
}

func NewFactoryB() AbstractFactory {
  return FactoryB{}
}


// ----------------------------------------------
type FactoryID struct {
  A       FactoryA
  B       FactoryB
}

func NewFactoryID(id string) AbstractFactory {
  factoryId := new(FactoryID)
  switch id {
  case "A":
    return factoryId.A
  case "B":
    return factoryId.B
  }
  return nil
}









