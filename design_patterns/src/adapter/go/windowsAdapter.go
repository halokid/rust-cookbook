package main

import "fmt"

// TODO: this is a adapter special for `Windows{}`

type WindowsAdapter struct {
  windowMachine *Windows
}

func (w *WindowsAdapter) InsertIntoLightningPort() {
  fmt.Println("Adapter converts Lightning signal to USB.")
  w.windowMachine.InsertIntoUSBPort()
}
