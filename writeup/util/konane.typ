
// render a Konane board
#let konane(str, tile-size: 1em) = {
  let str_rows = str.split("\n").map(v => v.trim())
  let width = calc.max(..str_rows.map(r => r.len()))
  let height = str_rows.len()

  grid(
    stroke: 1pt + black,
    rows: height * (tile-size + 6pt,),
    columns: width * (tile-size + 6pt,),
    inset: 3pt,
    ..str_rows.map(
      row => range(width).map(
        x => {
          let cell = row.at(x, default: "_")
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
          }
        }
      )
    ).flatten()
  )
}
