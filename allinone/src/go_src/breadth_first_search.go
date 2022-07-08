package main

import (
  "fmt"
  "sync"
)

type NodeQueue struct {
  nodes   []Node
  lock    sync.RWMutex
}

func (g *Graph) BFS(f func(node *Node)) {
  g.lock.Lock()
  defer g.lock.Unlock()

  // initialize queue
  q := NewNodeQueue()
  // get the first node of graph into queue
  head := g.nodes[0]
  fmt.Printf("The initialize g.nodes -->>> %+v\n", g.nodes)
  q.Enqueue(*head)
  // marking the node has already be visted
  visited := make(map[*Node]bool)
  visited[head] = true
  // traverse all nodes until queue is null
  i := 0
  for {
    i++
    if q.IsEmpty() {
      break
    }
    node := q.Dequeue()
    visited[node] = true
    nexts := g.edges[*node]
    // put all not yet visted nodes into queue
    for _, next := range nexts {
      // if noed has been visited
      if visited[next] {
        continue
      }
      q.Enqueue(*next)
      visited[next] = true
    }
    // node process callback
    if f != nil {
      f(node)
    }
    fmt.Printf("index %+v q.nodes -->>> %+v\n", i, q.nodes)
    fmt.Printf("-------------------------------\n")
  }
  fmt.Printf("The BFS path node queue -->>> %+v\n", g.nodes)
  fmt.Printf("The BFS path edges -->>> %+v\n", g.edges)
}

// create nodes queue
func NewNodeQueue() *NodeQueue {
  q := NodeQueue{}
  q.lock.Lock()
  defer q.lock.Unlock()
  q.nodes = []Node{}
  return &q
}

// push in queue
func (q *NodeQueue) Enqueue(n Node) {
  q.lock.Lock()
  defer q.lock.Unlock()
  q.nodes = append(q.nodes, n)
}

// pop out the first node of queue
func (q *NodeQueue) Dequeue() *Node {
  q.lock.Lock()
  defer q.lock.Unlock()
  node := q.nodes[0]
  fmt.Printf("Dequeue node -->>> %+v\n", node)
  q.nodes = q.nodes[1:]
  return &node
}

// check the queue is empty
func (q *NodeQueue) IsEmpty() bool {
  q.lock.Lock()
  defer q.lock.Unlock()
  return len(q.nodes) == 0
}







