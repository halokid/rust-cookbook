package main

import "log"

type Mac struct {

}

func (m *Mac) InsertIntoLightningPort() {
  log.Println("Lightning connector is plugged into mac machine.")
}