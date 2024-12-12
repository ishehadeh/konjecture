#import "../util/konane.typ": konane

= Known Facts

This section is a list of known facts about the game that informed our analysis.

== Space Complexity

Konane is PSPACE-Complete @hearnAmazonsKonaneCross2009, meaning it takes a polynomial amount of space relative to its input size, and any other problem in PSPACE can be solved in polynomial time if Konane can be solved in polynomial time.

== Existence of Arbitrary Nim Dimension
If there is a $*n$ position on an $l_n times c_n$ board, then there is a $*m$ position on an $l_m times c_m$ board such that:
$
l_m &= l_n + c_n + 2\
c_m &= l_n + 3
$

The base case is $l_2 = 7$, $c_2 = 6$. @santosKonaneHasInfinite2008

== Solid Linear Pattern

A solid linear pattern is series of $N$ alternating white and black pieces, with jumps allowed on either side.
#figure(caption: $N = 7$, konane("_xoxoxox_"))

The normal form for any solid linear pattern is:

$
-j &"  if " N = 2j + 1\
0 &"  if " N = 4j\
* &"  if " N = 4j + 2
$

Source: Ernst, _Playing Konane Mathematically_ @ernstPlayingKonaneMathematically1995

== Solid Linear Pattern With Tail

We say a solid linear pattern has a tail of length $M$, if there are $M$ alternating pieces below the left-most piece in the solid linear pattern.
#figure(caption: [Linear with Tail $N = 7, M = 2$], konane("\n_oxoxoxo_\n_x\n_o\n"))

=== Tail Length 1

Given a solid linear pattern of length $N$ with a tail of length $1$, the normal form is

$
*              &"  if " N in {2, 3}\
{* | arrow.b}  &"  if " N = 4\
{* | -2j + 2}  &"  if " N = 4j, "      where" j > 1\
{2j-1 | *}     &"  if " N = 4j + 1, " where" j > 0\
{* | -2j + 1}  &"  if " N = 4j + 2, " where" j > 0\
{2j | 0}       &"  if " N = 4j + 3, " where" j > 0\
$

Source: Ernst, _Playing Konane Mathematically_ @ernstPlayingKonaneMathematically1995


== Solid Linear Pattern with Offset Tail

A solid linear pattern of length $N$ with _offset_ tail of length $M$ is identical to a solid linear pattern with tail, but with the top left piece removed.
#align(center)[
  #grid(columns: 2, gutter: 3em,
    figure(caption: [Linear with Offset Tail $N = 7, M = 2$], konane("\n__xoxoxo_\n_x\n_o\n")),
    align(horizon)[
      $
        arrow.b  &"  if " N = 3\
        j - 1  &"  if " N = 2j, "      where" j > 0\
        *      &"  if " N = 4j + 1, " where" j > 0\
        0      &"  if " N = 4j + 3, " where" j > 0\
      $
    ]
  )
]
Source: Ernst, _Playing Konane Mathematically_ @ernstPlayingKonaneMathematically1995

== Somewhat Solid Linear Patterns

Let $S(a, b, c, ...)$ be a series of solid linear patterns beginning with a white tile, each separated by a space.

#figure(caption: $S(2, 3, 2)$, konane("_xo_xox_xo_"))

Quick Facts @nowakowski1xnKonaneSummary2010:

- $S(2a + 1, 2b + 1, 2c + 1) = a + b + c$
- $
S(2a + 1, 2b + 1, 2c) = S(2a + 1, 2(b + 1)) + S(2k - 2) = cases(star (i - (b + 1) + (c - 1)) "  if " a > b + 1,
star (2^(a + (b + 1) - 1) + (k - 1))  " if " a <= b + 1)
$

== Penetration

The penetration of a particular Konane configuration is how far its members are able to move. Rectangular configurations have a maximum penetration of $4$ @ernstPlayingKonaneMathematically1995.

== Patterns in Real Play

This is a living list of patterns observed in real play, and general facts about the game:

- When the game was first documented by European settlers the most common board size was $14 times 17$, although anything from $8 times 8$, to $13 times 20$ was also used. Modern games are often played on an $18 times 18$ board @nowakowski1xnKonaneSummary2010.
- "In real Konane games, play frequently proceeds to the corners after center of the board has been largely cleared" @ernstPlayingKonaneMathematically1995

