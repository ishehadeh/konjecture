
#let alpha = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"

#let fade-out(dir, color) = gradient.linear(color.transparentize(100%), color, angle: dir,).sharp(2)
#let open-border(color, width, up: true, down: true, left: true, right: true) = (x, y, w, h) => {
  if x == 0 and left { fade-out(0deg, color)}
  else if y == 0 and up { fade-out(90deg, color) }
  else if x == w - 1 and right { fade-out(180deg, color) }
  else if y == h - 1 and down { fade-out(270deg, color) }
  else { color } + width
}
// render a Konane board
#let konane(str, tile-size: 1em, inset: 3pt, board-grid: 1pt + black,  hlabel: (n) => n + 1, vlabel: (n) => alpha.at(n)) = {
  let str_rows = str.trim().split("\n").map(v => v.trim())
  let width = calc.max(..str_rows.map(r => r.len()))
  let height = str_rows.len()

  grid(
    rows: (if hlabel != none { (auto,) } else { () }) + (tile-size + inset * 2,) * height,
    columns: (if vlabel != none { (auto,) } else { () }) + (tile-size + inset * 2,) * width,
    inset: inset,
    ..if vlabel != none { ([],) } else { () },
    ..if hlabel != none {
      range(0, width).map(i => align(center)[#hlabel(i)])
    } else {
      ()
    },
    ..str_rows.enumerate().map(
      ((row_i, row)) => (
        ..if vlabel != none {
          (align(horizon)[#vlabel(row_i)],)
        } else {
          ()
        },
        ..range(width).map(
          x => {
            let cell = row.at(x, default: "_")

            grid.cell(stroke: if type(board-grid) == "function" { board-grid(x, row_i, width, height) } else { board-grid },
              if cell == "x" {
                circle(fill: black, width: tile-size, height: tile-size)
              } else if cell == "o" {
                circle(fill: white, stroke: black + 1pt, width: tile-size, height: tile-size)
              } else if cell == "X" {
                circle(fill: black, stroke: red + 2pt, width: tile-size, height: tile-size)
              } else if cell == "O" {
                circle(fill: white, stroke: red + 2pt, width: tile-size, height: tile-size)
              } else if cell == "_" {
                box(width: tile-size, height: tile-size)
              } else {
                panic("invalid cell: '" + cell + "'")
              })
          }),
      )
    ).flatten()
  )
}
