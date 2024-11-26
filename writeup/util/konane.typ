

// render a Konane board
#let konane(str, tile-size: 1em) = {
  let alpha = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
  let str_rows = str.trim().split("\n").map(v => v.trim())
  let width = calc.max(..str_rows.map(r => r.len()))
  let height = str_rows.len()

  grid(
    rows: height + 1,
    columns: width + 1,
    inset: 3pt,
    [],
    ..range(0, width).map(i => [#{i + 1}]),
    ..str_rows.enumerate().map(
      ((row_i, row)) => (
        alpha.at(row_i),
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
