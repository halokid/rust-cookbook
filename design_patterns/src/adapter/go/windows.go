package main

import "log"

type Windows struct {
}

func (w *Windows) InsertIntoUSBPort() {
  log.Println("USB connector is plugged into windows machine.")
}
