package main

import (
  "fmt"
  "sync"
)

type Node struct {
  value int
}

type Graph struct {
  nodes     []*Node
  edges     map[Node][]*Node
  lock      sync.RWMutex
}

// add node
func (g *Graph) AddNode(n *Node) {
  g.lock.Lock()
  defer g.lock.Unlock()
  g.nodes = append(g.nodes, n)
}

// add edge
func (g *Graph) AddEdge(u, v *Node) {
  g.lock.Lock()
  defer g.lock.Unlock()

  if g.edges == nil {
    g.edges = make(map[Node][]*Node)
  }

  g.edges[*u] = append(g.edges[*u], v)    // build the edge from u->v
  g.edges[*v] = append(g.edges[*v], u)     // because undirected graph, so exist v->u edges
}

// output graph
func (g *Graph) String() {
  g.lock.Lock()
  defer g.lock.Unlock()

  str := ""
  for _, iNode := range g.nodes {
    str += iNode.String() + " -> "
    nexts := g.edges[*iNode]
    for _, next := range nexts {
      str += next.String() + " "
    }
    str += "\n"
  }
  fmt.Println(str)
}

func (n *Node) String() string {
  return fmt.Sprintf("%+v", n.value)
}

func main() {
  g := Graph{}
  n1, n2, n3, n4, n5 := Node{1}, Node{2}, Node{3}, Node{4}, Node{5}

  g.AddNode(&n1)
  g.AddNode(&n2)
  g.AddNode(&n3)
  g.AddNode(&n4)
  g.AddNode(&n5)

  g.AddEdge(&n1, &n2)
  g.AddEdge(&n1, &n5)

  g.AddEdge(&n2, &n3)
  g.AddEdge(&n2, &n4)
  g.AddEdge(&n2, &n5)

  g.AddEdge(&n3, &n4)

  g.AddEdge(&n4, &n5)

  g.String()

  g.BFS(func(node *Node) {
    fmt.Printf("[Current Traverse Node] -->>> %+v\n", node)
  })
}













