// mi: identifier (Examples: variable, function name, symbolic constant)
// mn: numeric literal (Examples: integer, decimal number)
// mo: operator (Examples: binary operation, binary relation, summation symbol)
// mtext: generic text (Examples: short words in formulas )

// mrow
// msub
// msup
// msubsup
// munder
// mover
// munderover

// mtable
// mtr
// mtd: columnspan, rowspan

enum MathType {
    Identifier, // Symbols like pi, dot, or RR
    Numeric,
    Operator,
    Variables, // Single letters are always displayed as is.
               // Multiple letters, however, are interpreted as variables and functions.
               // To display multiple letters verbatim, you can place them into quotes
               // and to access single letter variables, you can use the hash syntax.
}
