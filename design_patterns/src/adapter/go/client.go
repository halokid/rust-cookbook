package main

import "log"

type Client struct {
}

func (c *Client) InsertLightningConnectorIntoComputer(com Computer) {
  log.Println("-->>> Client insert Lightning connector into computer.")
  com.InsertIntoLightningPort()
  log.Println()
}
