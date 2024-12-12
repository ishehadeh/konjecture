

// render a Konane board
#let konane(str, tile-size: 1em,  hlabel: (n) => n, vlabel: (n) => n) = {
  let str_rows = str.trim().split("\n").map(v => v.trim())
  let width = calc.max(..str_rows.map(r => r.len()))
  let height = str_rows.len()


  grid(
    rows: (if hlabel != none { (auto,) } else { () }) + (tile-size + 6pt,) * height,
    columns: (if vlabel != none { (auto,) } else { () }) + (tile-size + 6pt,) * width,
    inset: 3pt,
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

            grid.cell(stroke: 1pt + black,
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
