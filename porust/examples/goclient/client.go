package main

import (
  "log"
  "reflect"

  pb "./proto"
  "golang.org/x/net/context"
  "google.golang.org/grpc"
)

const (
  //address  = "127.0.0.1:18080"
  //address  = "172.21.29.32:18080"
  address  = "192.168.1.101:18080"
)

func main() {
  // Set up a connection to the server.
  conn, err := grpc.Dial(address, grpc.WithInsecure())
  if err != nil {
    log.Fatalf("did not connect: %v", err)
  }
  defer conn.Close()
  c := pb.NewPorsClient(conn)

  // Contact the server and print out its response.
  /**
  name := `{"name": "HaloKid"}`
  if len(os.Args) > 1 {
    name = os.Args[1]
  }
  rsp, err := c.Invoke(context.Background(), &pb.Req{ Reqdata: name })
  */

  // todo: call porust.say_hi
  reqData := `{"call": "porust.say_hi", "data": {"name": "halokid"}}`
  rsp, err := c.Invoke(context.Background(), &pb.Req{ Reqdata: reqData })

  if err != nil {
    log.Fatalf("could not greet: %v", err)
  }
  log.Printf("say_hi---rsp type: %+v, struct: %+v, val: %+v", reflect.TypeOf(rsp), rsp, rsp.Rspdata)

  // todo: call read data
  reqData = `{"call": "porust.read_data", "data": {"key": "halokid"}}`
  rsp, err = c.Invoke(context.Background(), &pb.Req{ Reqdata: reqData })
  if err != nil {
    log.Fatalf("could not greet: %v", err)
  }
  log.Printf("read_data---rsp type: %+v, struct: %+v, val: %+v", reflect.TypeOf(rsp), rsp, rsp.Rspdata)
}







