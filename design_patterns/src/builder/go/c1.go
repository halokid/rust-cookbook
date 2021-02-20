package main

import "log"

/*
建造者模式
 */

// todo: 1. 目标类
type Person struct {
  name    string      // 必须
  age     uint8       // 必须
  job     string
}

func NewPerson(name string, age uint8) Person {
  return Person{
    name: name,
    age:  age,
  }
}

func (p *Person) setJob(job string) {
  p.job = job
}

// -----------------------------------------------------
// todo: 2. 抽象构建者类
type Builder interface {
  buildJob()
  build() Person
}

// ----------------------------------------------------
// todo: 3. 实体构建者类
type AliceBuilder struct {
  obj     Person
}

func NewAliceBuilder() AliceBuilder {
  return AliceBuilder{
    obj:  NewPerson("Alice", 12),
  }
}

func (alice *AliceBuilder) buildJob() {
  alice.obj.setJob("Student")
}

func (alice *AliceBuilder) build() Person {
  return alice.obj
}

// todo: 4. 主导类 Director
type PersonDirector struct {
  build     Builder   // todo: 指向抽象构建类
}

func NewPersonDirector(b Builder) PersonDirector {
  return PersonDirector{
    build:  b,
  }
}

func (p *PersonDirector) buildx() Person {
  p.build.buildJob()        // todo: 这里只有方法调用， 不指定参数
  return p.build.build()
}

func main() {
  aliceBuilder := NewAliceBuilder()     // 1. 建立一个builder
  director := NewPersonDirector(&aliceBuilder)      // 2. 建立一个director， load builder参数
  alice := director.buildx()            // 3. 通过 director 来 build alice
  log.Printf("alice ---------------- %+v", alice)
}






