#import "../util/bib.typ": bib
#import "../util/theorems.typ": definition, example
#import "../util/konane.typ": konane

= Computation Analysis Konane


We were unable to find any open-source programs with a well-optimized implementation of Konane. But, computer-friendly grid game representations are well studied for games like Go, Chess and Checkers (@schaefferCheckersSolved2007).

== Implementation

#definition()[
  _Bit Field_: A bit field is a string of binary digits with a constant length. We write a bit fields right-to-left. Each bit has an associated index, beginning at $0$.

  Example: Given the bit field $B = 01101$, $B_0 = 1$, $B_1 = 0$, $B_2 = 1$, $B_3 = 1$, $B_4 = 0$, 
]


We use two bit fields to represent a game of Konane.
Given a board of width $W$ and height $H$, we use bit fields with $W times H$ elements. The first bit field (hereafter _BLACK_) has a $1$ at index $y * W + x$ if and only if the game has a black piece in cell $(x, y)$ (the top left cell is $(0, 0)$). Similarly, the bit field representing white (hereafter _WHITE_) has a $1$ at $y * W + x$ if and only if the game has a white piece in the cell $(x, y)$.


#figure(caption: "Bit Field Representation")[
  #grid(columns: 3, gutter: 1em,
    konane("xo_\n_xo"),
    align(center + horizon, [is represented as]),
    align(center + horizon)[
      $
        "BLACK" &= 100010\
        "WHITE" &= 010001
      $
    ]
  )
]

This format is compact, and it allows us to use only a few CPU instructions to calculate each player's moves. 


=== Testing <testing>

To ensure accurate results, this implementation must be carefully tested. 

The underlying bit-field representation is mostly tested using a method called property-based testing. These types of tests are concerned with a specific _property_, some check that theoretically always holds given a certain class of inputs, then we generate random input values and check the property to verify that it holds in practice.

The game's implementation has several specific tests, which come from simple cases (do we get the correct set of moves for a $2 times 2$ game, etc.), and property tests based on established facts about specific Konane patterns. We've also begun to write tests based on well-known Konane positions, primarily from _Playing Konane Mathematically_  @ernstPlayingKonaneMathematically1995. The first of these tests builds a series of alternating black and white pieces in a line, then puts it in normal form and compares the result with the expected value, which is outlined in _Playing Konane Mathematically_ @ernstPlayingKonaneMathematically1995.
Note although we only test these cases, the implementation is general enough to handle arbitrary games of Konane.

The test suite has expanded to the point we are reasonably confident about the program's accuracy. But, we'll continue to add unique test cases as they arise.

== Invariants


Most invariants are picked arbitrarily, they're numbers that "seems interesting". But facts about particular positions from prior research inform the ideas. For example, in _Konane has Infinite Nim Dimensions_ @santosKonaneHasInfinite2008, construction of new positions relies on a "focal point", a piece that can be captured in a few ways, and opens both players up to new moves. It suggests that the number of pieces that can be captured, and the number of moves for a given player may be interesting attributes. 

In the second week, we began implementing invariants.
The underlined items have been implemented.

The current list of invariants is:

- Average distance from each piece to the nearest border.
- Total possible moves.
- Number of unique pieces that can be captured.
- Counts of each tile state.
- Distance between the highest and lowest piece.
- Distance between the furthest left and furthest right piece.
- The nim-value of a game
- The number-value of a game

All of these can (except for the game values), can be calculated for a single player, or both.

#bib()