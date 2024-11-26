#import "../util/konane.typ": konane

= Conjecturing

Conjecturing (@larsonAutomatedConjecturingFajtlowiczs2016)
is a projectthat generates a series of true iequalities based on a table of objects with associated _invariants_.
In our case, an object is a single Konane position, and invariants include things like _number of moves_, _number of black pieces_, _number of white pieces_, etc.

#figure(caption: "Conjecturing Inputs")[
  #table(
    columns: 4,
    rows: 3, 
    table.header([*Games*], [*\# Black* ($I_1$)], [*\# White* ($I_2$)], [*\# Moves* ($I_3$)]),
    
    konane("xox\n_", cell-names: false),
    $2$, $1$, $0$, 
    
    konane("x_o\nxo", cell-names: false),
    $2$, $2$, $1$, 

    konane("ox\n_xo", cell-names: false),
    $2$, $2$, $2$, 
  )
]

We choose a single invariant to place on the left hand side of the inequality, and which operators to use.

