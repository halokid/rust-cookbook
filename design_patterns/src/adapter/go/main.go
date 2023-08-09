package main

/**
https://refactoringguru.cn/design-patterns/adapter/go/example
*/

func main() {
  client := &Client{}

  // for mac
  mac := &Mac{}
  client.InsertLightningConnectorIntoComputer(mac)

  // TODO: at first the `&Windows{}` can not directly use in func InsertLightningConnectorIntoComputer
  // TODO: must use some wrap to convert the `&Windows{}` make it can use in func InsertLightningConnectorIntoComputer
  // TODO: here is use pass it into a struct,  and this struct `&WindowsAdapter{}` is the `wrap`
  // for windows
  windowsMachine := &Windows{}
  windowsMachineAdapter := &WindowsAdapter{
    windowMachine: windowsMachine,
  }
  client.InsertLightningConnectorIntoComputer(windowsMachineAdapter)
}
