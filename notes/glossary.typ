#import "template.typ": *

#show: project.with(
  title: "Glossary",
  authors: (
    "Nikita Tikhonov",
  )
)

= Glossary

== Graph

A *graph* is a pair $G = (V, E)$ where $V$ is a set of *vertices* and $E$ is a set of *edges*.

A *directed* graph contains ordered pairs $(u, v)$ in $E$ (so $E subset.eq V^2$):

#figure(
  diagram(
    node-shape: circle,
    node-stroke: white,
    edge-stroke: white,
    {
      // positions
      let a = (0, 0)
      let b = (1, 0)
      let c = (0, 1)
      // arrows
      let arrow = "-}>"
      // nodes
      node(a, $a$)
      node(b, $b$)
      node(c, $c$)
      // edges
      edge(a, b, arrow)
      edge(b, c, arrow)
      edge(c, a, arrow)
    }
  ),
  caption: "A directed graph"
) <directed>

While an *undirected* graph contains unordered pairs ${u, v}$ in $E$:

#figure(
  diagram(
    node-shape: circle,
    node-stroke: white,
    edge-stroke: white,
    {
      // positions
      let a = (0, 0)
      let b = (1, 0)
      let c = (0, 1)
      // nodes
      node(a, $a$)
      node(b, $b$)
      node(c, $c$)
      // edges
      edge(a, b)
      edge(b, c)
      edge(c, a)
    }
  ),
  caption: "An undirected graph"
) <undirected>

If we define an _incidence function_ $phi: E -> V^2$ we obtain *multigraphs* $G = (V, E, phi)$:

#figure(
  diagram(
    node-shape: circle,
    node-stroke: white,
    edge-stroke: white,
    {
      // positions
      let a = (0, 0)
      let b = (1, 0)
      // arrows
      let arrow = "-}>"
      // nodes
      node(a, $a$)
      node(b, $b$)
      // edges
      edge(a, b, arrow, bend: 30deg)
      edge(b, a, arrow)
      edge(a, b, arrow, bend: -30deg)
    }
  ),
  caption: "A multigraph",
) <multigraph>

There are also *weighted* graphs or *networks*, where the _weight_ $w$ is assigned to each edge.

A *regular* graph is a graph where each vertex has the same _degree_.

A *complete* graph contains all possible edges:

#figure(
  diagram(
    node-shape: circle,
    node-stroke: white,
    edge-stroke: white,
    {
      // positions
      let a = (0, 0)
      let b = (1, 0)
      let c = (0, 1)
      let d = (1, 1)
      // nodes
      node(a, $a$)
      node(b, $b$)
      node(c, $c$)
      node(d, $d$)
      // edges
      edge(a, b)
      edge(a, c)
      edge(a, d)
      edge(b, c)
      edge(b, d)
      edge(c, d)
    }
  ),
  caption: "A complete graph",
)
