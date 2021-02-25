package main

import "log"

// 具体主体
type item struct {
  observerList   []observer
  name           string
  inStock        bool
}

func newItem(name string) *item {
  return &item{
    name: name,
  }
}

func (i *item) updateAvailability() {
  log.Printf("Item %s is now in stock\n", i.name)
  i.inStock = true
  i.notifyAll()     // todo: 执行通知
}

func (i *item) register(o observer) {
  i.observerList = append(i.observerList, o)
}

func (i *item) deregister (o observer) {
  i.observerList = removeFromslice(i.observerList, o)
}

func (i *item) notifyAll() {
  for _, oberver := range i.observerList {
    oberver.update(i.name)
  }
}

func removeFromslice(observerList []observer, observerToRemove observer) []observer {
  observerListLength := len(observerList)
  for i, observer := range observerList {
    if observerToRemove.getID() == observer.getID() {
      observerList[observerListLength-1], observerList[i] = observerList[i], observerList[observerListLength-1]
      return observerList[:observerListLength-1]
    }
  }
  return observerList
}



