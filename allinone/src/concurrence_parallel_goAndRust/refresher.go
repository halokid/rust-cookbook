package main

import (
    "fmt"
    "time"
)

func main() {
    go func() {
        for {
            fmt.Println("在另外一个协程里过去了一秒")
            time.Sleep(time.Second)
        }
    }()

    fmt.Println("等待goroutine 10秒, 然后主协程退出")
    time.Sleep(time.Second * 10)
}