#import "../util/konane.typ": konane

= Konane

Konane is an ancient Hawaiian game played on a rectangular grid. 
The size of this grid varies.
There are two players, one playing with black pieces, and the other white pieces.

== Konane Starting Phase

A game begins with black removing one of their corner pieces or one of their center pieces (@thompsonTeachingNeuralNetwork2005). For example, in @eg-start the game begins with black choosing one of the pieces outlined in red to remove. Once black removes a piece, white then chooses an adjacent piece to remove. For example, if black begins by removing D4, moving the position in @eg-start-t2, white can now remove any of the highlighted pieces. 

#grid(columns: 3, gutter: 3em,
  [
    #figure(caption: [Starting Position])[
      #konane("
        Xoxoxo
        oxoxox
        xoXoxo
        oxoXox
        xoxoxo
        oxoxoX
      ")
    ]<eg-start>
  ],

  [
    #figure(caption: "Turn 1",
        konane("
          xoxoxo
          oxoxox
          xoxOxo
          oxO_Ox
          xoxOxo
          oxoxox
        ")
    )<eg-start-t2>
  ],

  [
    #figure(caption: "Turn 2",
        konane("
          xoxoxo
          oxoxox
          xoxoxo
          ox__ox
          xoxoxo
          oxoxox
        ")
    )<eg-start-t3>
  ]
),

Once white removes a piece, normal play begins.

== Konane Normal Play

On each player's turn, they must capture at least one of their opponent's pieces. The first player who is unable to capture any pieces loses.

A player may capture a piece of the opposing color is captured by _jumping_ over it using one of their pieces into an empty space. @eg-capture shows white capturing a black piece by moving from A3 to A1.

#figure(caption: "Capture")[
  #grid(column-gutter: 2em, row-gutter: 1em, columns: 2, rows: 2,
    konane("_xO_\n_"),
    konane("O___\n_"),
    align(center, [#h(16pt)Turn 1]),
    align(center, [#h(16pt)Turn 2]))
]<eg-capture>

Multiple captures can be made in a single direction. For example, beginning from _Start_ in @eg-capture2, if black plays first they can move to _Option 1_, _Option 2_, or _Option 3_.

#figure(caption: "Capture")[
  #grid(column-gutter: 2em, row-gutter: 1em, columns: 4, rows: 2,
    konane("___\n_X_\n_o_\n___\n_o_\n__\n_o_\n___"),
    konane("___\n__\n__\n_X_\n_o_\n__\n_o_\n__"),
    konane("___\n__\n__\n__\n__\n_X_\n_o_\n__"),
    konane("___\n__\n__\n__\n__\n__\n__\n_X_"),
    align(center, [#h(16pt)Start]),
    align(center, [#h(16pt)Option 1]),
    align(center, [#h(16pt)Option 2]),
    align(center, [#h(16pt)Option 3]))
]<eg-capture2>

