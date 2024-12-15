#import "../util/konane.typ": *
#import "../util/theorems.typ": theorem
#let LT = "LT"
#let LOT = "LOT"
#let SLP = "SLP"

#let LT1 = $LT_1$
#let LOT1 = $LOT_1$

#let lot(n) = konane("_\n__" + range(1, n).map((i) => if calc.rem(i, 2) != 0 { "x" } else { "o" }).join("") + "_\n_x")
#let knsmall = konane.with(tile-size: 6pt, inset: 2pt, vlabel: none, hlabel: none, board-grid: 0.3pt + black)

#let lotsm(n) = knsmall("_\n__" + range(1, n).map((i) => if calc.rem(i, 2) != 0 { "x" } else { "o" }).join("") + "_\n_x")


= Linear with Offset Tail
  In this section, we'll analyze the outcome class of $LOT1(n)$. Games in this pattern have a black stone in position C3, and $n - 1$ alternating black and white pieces in row B, beginning at cell B3 and moving right.

  #grid(columns: 3, gutter: 1fr)[
    #figure(caption: $LOT1(5)$)[
      #lot(5)
    ]
  ][
    #figure(caption: $LOT1(2)$)[
      #lot(2)
    ]
  ][
    #figure(caption: $LOT1(3)$)[
      #lot(3)
    ]
  ]
  
  First, the early positions have no moves available, so $LOT1(1) = LOT1(2) = 0$.

  $LOT1(3) = #lotsm(3) = {knsmall("____x\n_x")|knsmall("_o_\n_x")} = {0|*} = arrow.t$
  
  $LOT1(4) = #lotsm(4) ={knsmall("_o__x\n_x"),knsmall("__x__o\n_x") |} = {|*,0} = -1$.

  To find the general case, we'll look at the even and odd cases individually.

== $LOT1(n)$ When $n$ is Even

#grid(columns: 2, gutter: 2em)[
Consider a game $LOT1(2i)$, because there are an even number of stones, we know the right-most stone is black. So, black has no moves available, and white has two: they can capture the left-most black stone in B3, moving the game to $SLP(2i - 3) + SLP(2)$ (the white and black stones in column 2 will be out of reach for the solid linear pattern in columns 5+). Or, white can capture the right-most white stone (7B in @lot6), moving the game to $LOT1(2(i - 1))$.
][
  #figure(caption: $LOT1(6)$)[
    #lot(6)
  ]<lot6>
]

By @slp-outcome we know $SLP(2i - 3) + SLP(2) = (-i + 1) + *$$$. Now, we have $LOT1(6) = {|-1+*, LOT1(4)} = {|-1+*,-1}$. Because $-1$ dominates $-1 + *$, $LOT1(6) = {|-1} = -2$. Assuming $LOT1(2i) = -(i - 1)$, then $LOT1(2(i + 1)) = {|-(i - 1) + *, -(i - 1)} = {|-(i - 1)} = -i$. Because $LOT1(2(3)) = -2$, $LOT1(2(2)) = -1$,  by induction it follows that $LOT1(2(i + 1)) = -i$.

Therefore, in the general case we can say $LOT1(2i) = -(i - 1)$, for $i >= 1$.

== $LOT1(n)$ When $n$ is Odd

#grid(columns: 2, gutter: 2em)[
Consider a game $LOT1(2j + 1)$, the right-most stone is white. So, black and white both have a single move available. White can capture the black stone in B3, moving the game to $SLP(2) +SLP(2(j - 1))$, and black can capture the right-most white stone, moving the game to $LOT_1(2j - 1)$.
][
  #figure(caption: $LOT1(5)$)[
    #lot(5)
  ]<lot5>
]

So, we have the game $LOT1(2j + 1) = {LOT1(2j - 1)|SLP(2) + SLP(2(j - 1))}$. by @slp-outcome, we know

$
"SLP"(2j) = cases(
      space 0 &"  if j is even",
      space * &"  if j is odd"
    )
$

So let's split $LOT1$ into even and odd cases for $j$:

$
  LOT1(2j + 1) &= cases(
      {LOT1(2(j - 1) + 1)| * + *} &"  if j is even",
      {LOT1(2(j - 1) + 1)| * + 0} &"  if j is odd"
  )\
  &= cases(
      {LOT1(2(j - 1) + 1)| 0} &"  if j is even",
      {LOT1(2(j - 1) + 1)| *} &"  if j is odd"
  )
$

Recall $LOT1(3) = arrow.t$, so $LOT1(5) = {arrow.t|0} = *$, and $LOT1(7) = {* | *} = 0$. By the above recursive definition of $LOT1(2j + 1)$, we can see this pattern repeat, leading to the simplified definition:
$
  LOT1(2j + 1) &= cases(
      * &"  if j is even",
      0 &"  if j is odd"
  )
$

== Putting it Together

#theorem(name: [Outcome of $LOT1$(n)])[
$
  LOT1(n) &= cases(
      * &"  if" n = 4j + 1,
      0 &"  if" n = 4j + 3,
      -(j - 1) &"  if" n = 2j,
  )
$
]<lot-outcome>

The first 2 cases are from Section 6.2, and the last is from section 6.1.
