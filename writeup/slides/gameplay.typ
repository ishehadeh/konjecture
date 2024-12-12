#import "../util/konane.typ": *

#let alpha = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
#let hlabel = (n) => n + 1
#let vlabel = (n) => alpha.at(n)

== Example: Move

#align(center)[
  #konane("
    xoxo_o
    o__x_x
    xo__xO
    ox__ox
    xoxoxo
  ", tile-size: 2em, hlabel: hlabel, vlabel: vlabel)
]

== Example: Move

#align(center)[
  #konane("
    xoxo_o
    o__x_x
    xo_OX_
    ox__ox
    xoxoxo
  ", tile-size: 2em, hlabel: hlabel, vlabel: vlabel)
]

== Example: Move

#align(center)[
  #konane("
    xoxo_o
    o__x_x
    xo_O__
    ox__ox
    xoxoxo
  ", tile-size: 2em, hlabel: hlabel, vlabel: vlabel)
]




== Example: Multiple Jumps

#align(center)[
  #konane("
    _______
    _Xo_o__
    _______
  ", tile-size: 2em, hlabel: hlabel, vlabel: vlabel)
]

== Example: Multiple Jumps

#align(center)[
  #konane("
    _______
    __OXo__
    _______
  ", tile-size: 2em, hlabel: hlabel, vlabel: vlabel)
]

== Example: Multiple Jumps

#align(center)[
  #konane("
    _______
    __OXo__
    _______
  ", tile-size: 2em, hlabel: hlabel, vlabel: vlabel)
]

== Example: Multiple Jumps

#align(center)[
  #konane("
    _______
    ___Xo__
    _______
  ", tile-size: 2em, hlabel: hlabel, vlabel: vlabel)
]

== Example: Multiple Jumps

#align(center)[
  #konane("
    _______
    ____OX_
    _______
  ", tile-size: 2em, hlabel: hlabel, vlabel: vlabel)
]

== Example: Multiple Jumps

#align(center)[
  #konane("
    _______
    _____X_
    _______
  ", tile-size: 2em, hlabel: hlabel, vlabel: vlabel)
]