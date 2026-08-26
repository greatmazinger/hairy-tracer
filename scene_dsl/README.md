# Scene DSL

A domain-specific language for building constructive solid geometry (CSG) scenes for Hairy Tracer.

## CSG Operator Precedence

To cleanly support infix operations for Constructive Solid Geometry, the following operator precedence table is used. The operations are evaluated from highest precedence to lowest.

1. **Intersection (`&`)**: Highest precedence. Intersections bind tighter than anything else, similar to multiplication. Left-associative.
2. **Difference (`-`)**: Middle precedence. Evaluates after intersections. Left-associative.
3. **Union (`|`)**: Lowest precedence. Evaluates last, similar to addition. Left-associative.

This means `a - b | c & d` parses as `(a - b) | (c & d)`.

## AST Structure
CSG expressions will be parsed using Pratt parsing (precedence climbing) according to this precedence table.
