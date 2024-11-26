#import "../util/konane.typ": konane

= Conjecturing

Conjecturing (@larsonAutomatedConjecturingFajtlowiczs2016)
is a projectthat generates a series of true iequalities based on a table of objects with associated _invariants_.
In our case, an object is a single Konane position, and invariants include things like _number of moves_, _number of black pieces_, _number of white pieces_, etc.

#figure(caption: "Conjecturing Inputs")[
  #table(
    columns: 4,
    rows: 3, 
    table.header([*Games*], [*\#Black* ($I_1$)], [*\#White* ($I_2$)], [*\#Moves* ($I_3$)]),
    
    konane("xox\n_", cell-names: false),
    $2$, $1$, $0$, 
    
    konane("x_o\nxo", cell-names: false),
    $2$, $2$, $1$, 

    konane("ox\n_xo", cell-names: false),
    $2$, $2$, $2$, 
  )
]<eg-conjecturing-data>

We choose a single invariant to place on the left hand side of the inequality, and which operators to use.
For example, given the data in @eg-conjecturing-data, we could instruct _Conjecturing_ to use $I_2$ on the left hand side of $<=$, and create expressions using the operators $A + B, A * B, A + 1, sqrt(A), max(A, B)$. A few of the possible results are shown in @eg-conjecturing-output.

#figure(caption: "Conjecturing Output")[
  #align(center + horizon,
  $
    I_2 &<= max(I_1, I_2)\
    I_2 &<= I_1 + I_2\
    I_2 &<= I_1 + 1\
    I_2 &<= I_1 * I_2 + 1 +1

  $)
]<eg-conjecturing-output>
