#import "template.typ": *

#show: project.with(
  title: "Events",
  authors: (
    "Nikita Tikhonov",
  )
)

#diagram(
  node-shape: circle,
  node-stroke: white,
  edge-stroke: white,
  {
  // positions
  let a = (0, 0)
  let b = (-1, 1)
  let c = (1, 1)
  let d = (-2, 2)
  let e = (2, 2)
  let f = (-1, 3)
  let g = (1, 3)
  let h = (3, 3)
  // arrows
  let arrow = "-}>"
  // nodes
  node(a, $a$)
  node(b, $b$)
  node(c, $c$)
  node(d, $d$)
  node(e, $e$)
  node(f, $f$)
  node(g, $g$)
  node(h, $h$)
  // edges
  edge(a, b, arrow) // tree
  edge(a, c, arrow) // tree
  edge(a, h, arrow, bend: 45deg, stroke: green) // forward
  edge(b, d, arrow) // tree
  edge(c, e, arrow) // tree
  edge(d, f, arrow) // tree
  edge(e, g, arrow) // tree
  edge(e, h, arrow) // tree
  edge(e, d, arrow, stroke: blue) // cross
  edge(f, b, arrow, bend: 105deg, stroke: red) // back
})
