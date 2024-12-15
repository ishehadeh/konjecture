
= Linear with Tail

#let lt(n) = konane("_\n_" + range(1, n).map((i) => if calc.rem(i, 2) == 0 { "x" } else { "o" }).join("") + "_\n_x_\n_")

To show how a position in Konane is analyzed using Combinatorial Game Theory we'll examine $LT_1(n)$.

The linear with tail pattern places a black stone in position C2, and $n - 1$ stones in row B, beginning with B2 and continuing to the right, alternating colors with every piece placed.
For example take $LT_1(5)$, $LT_1(2)$, and $LT_1(7)$:

#grid(columns: 3, gutter: 1fr)[
  #figure(caption: $LT_1(5)$)[
    #lt(5)
  ]
][
  #figure(caption: $LT_1(2)$)[
    #lt(2)
  ]
][
  #figure(caption: $LT_1(7)$)[
    #lt(7)
  ]
]

== Small Patterns

We'll examine the first four $LT_1$ patterns individually.
#align(center)[
  #grid(columns: 4, gutter: 2em)[
    #figure(caption: $LT_1(1)$)[
      #lt(1)
    ]
  ][
    #figure(caption: $LT_1(2)$)[
      #lt(2)
    ]
  ][
    #figure(caption: $LT_1(3)$)[
      #lt(3)
    ]
  ][
    #figure(caption: $LT_1(4)$)[
      #lt(4)
    ]
  ]
]
$LT_1(1) = 0$, since neither player has a move. $LT_1(2) = {0|0} = *$, since both players can remove their opponent's only piece. In a similar vein, black can remove white's only piece in $LT_1(3)$, and both of white's options place their stone out of reach. So $LT_1(3) = *$ as well.


$LT_1(4)$ is slightly more complex. White has the sole option to move the game to $*$, by moving from B2 to C2. Black has more options: B3 to B5 moves the game to $*$; C2 to  A2 moves the game to ${0|*}$; and B3 to B1 moves the game to $0$.


#align(center)[
  #figure(caption: [$LT_1(4)$ Game Tree])[
    #grid(columns: 2, gutter: 3em)[
      ${#h(0.5em) konane("_\n_o__x\n_x") #h(1em) konane("_\nx__o\n_x") #h(1em) konane("_x\n__xo_\n_") #h(1em) #math.stretch(math.bar.v, size: 700%) #h(1em) konane("__\n__xo_\n__\n_o") #h(0.5em)}$
    ]
]
]
Therefore,  $LT_1(4) = {*,0,arrow.t|*} = {arrow.t|*}$.

== Even Games

#grid(columns: (2fr, 1fr), gutter: 1em)[
  First, consider a game $LT_1(2i)$, where $j > 2$.
  These games will always end with a white stone, so white's only option is to move from B2 to D2. This move leaves a solid linear pattern in row B, with length $2(i - 1)$, and white stone in D2. Because pieces in a horizontal linear pattern can only move horizontally, it follows that this game is equivalent to $"SLP"(2(i - 1))$. By @slp-outcome we know that $"SLP"(2(i - 1)) = *$ if $i$ is even, and $0$ otherwise.
][
  #figure(caption: $i = 3$)[
    #lt(6)
  ]
]
On the other hand, black has  a similar set of moves to those we saw in $LT_1(4)$.
They can capture the right-most white stone, shortening the pattern to $LT_1(2(i - 1))$. They can capture B2 via B3, moving the game to $"SLP"(2(i - 1) - 1)$, or they can capture B2 via C2, moving the game to $"LOT"_1(2i)$. In short, we define $LT1(2i)$ as

$
LT_1(2i) = {LOT1(2i), space LT1(2(i - 1)), space SLP(2(i - 1) - 1) space | space SLP(2(i - 1))}
$

== Odd Games

#grid(columns: (2fr, 1fr), gutter: 1em)[
  Next, consider a game $LT_1(2j + 1)$, where $j > 1$.
  These games will always end with a black stone. White has the option to capture this stone, moving the game to $LT1(2j - 1)$. Or, white can capture the stone in C2, moving the game to $SLP(2(j - 1))$.

  Black has a similar move set to $LT1(3)$: they can capture B2 via their stone in B3, moving the game to $SLP(2(j - 1))$; or they can capture B2 via C3, moving the game to $LOT(2j)$. 
][
  #figure(caption: $j = 2$)[
    #lt(5)
  ]
]

$
LT_1(2j + 1) = {LOT1(2j), space SLP(2(j - 1)) space | space LT1(2j - 1), space SLP(2(j - 1))}
$

== Putting it Together

So far we have found concrete outcomes of the first $LT1(0)$ to $LT1(4)$, and the move sets for the general case.
#grid(columns: 2, gutter: 2em,
  grid.cell(rowspan: 2)[
    #table(align: center + horizon, columns: 2,
      table.header($n$, $LT1(n)$),
      $1$, $0$,
      $2$, $*$,
      $3$, $*$,
      $4$, ${arrow.t|*}$
    )
  ],
  pad(top: 8pt)[
  $LT_1(2i) = &{ space LOT1(2i), space LT1(2(i-1)), space SLP(2(i - 1) - 1) \
              &| SLP(2(i - 1)) space }$],

  $LT_1(2j + 1) = &{ space LOT1(2j), space SLP(2(j - 1)) space \
                  &| space LT1(2j - 1), space SLP(2(j - 1)) space}$,
)

Using @lot-outcome and @slp-outcome we can simplify the general case:

  $LT_1(2i) = &{ space LOT1(2i), space LT1(2(i-1)), space SLP(2(i - 1) - 1) \
              &| SLP(2(i - 1)) space }$],

  $LT_1(2j + 1) = &{ space LOT1(2j), space SLP(2(j - 1)) space \
                  &| space LT1(2j - 1), space SLP(2(j - 1)) space}$,
$
  LT1(n) &= cases(
      {*, LT1(2(i - 1)), SLP(2(i - 1) - 1)} &"  if" n = 4i + 1,
      0 &"  if" n = 4j + 3,
      -(j - 1) &"  if" n = 2j,
  )
$