# Prompt: Scene DSL — lexer, parser, type checker, elaborator, JSON emitter
## Context

The gear scene is ~800 lines of hand-nested JSON for what's conceptually "a disk, minus a bore, plus 24 teeth around the rim." Two real bugs (transform composition order, coincident-surface flush-fit) hid inside that repetition — not because the ideas were hard, but because hand-copy-pasting the same structure 24 times with slightly different numbers is exactly where mistakes hide. This task builds a small external DSL that compiles down to the existing scene JSON schema, so scenes like this can be written as loops and expressions instead of copy-pasted trees. The Rust render engine itself is not touched at all — this is a new, separate front-end tool.

**Design decisions already made, not open for reconsideration:**
- External DSL with real syntax, not an embedded builder API
- Hand-rolled lexer and recursive-descent parser, using **Pratt parsing (precedence climbing)** specifically for the CSG infix operators (`|` union, `&` intersect, `-` difference) — this is the principled technique for this exact problem, not ad hoc precedence logic
- The language is **deliberately non-Turing-complete**: `for` loops only range over compile-time-constant integer bounds, there is no general recursion. This guarantees compilation terminates by construction — a small, total language, not an accidentally general one.
- **Angles are a distinct type from plain scalars**, with unit-suffixed literals (`15deg`). The type checker must reject mixing `Angle` and `Scalar` — this structurally prevents the degrees/radians confusion that's caused real bugs earlier in this project (the Sponza FOV saga). Internally, `Angle` can be a compile-time-only newtype erased to a bare radian `f64` during elaboration — the distinction has zero runtime cost, same spirit as Rust's own ownership checking.

**Illustrative syntax** (not a spec to match exactly — the agent should propose and document the precise grammar):

```
material gold {
  diffuse: [0.8, 0.6, 0.1]
  ambient: 0.1
  specular: 0.5
  shininess: 80
}

let tooth = cube(min: [-0.25, -0.5, -0.25], max: [0.25, 0.5, 0.25])
let disk  = cylinder(center: [0,0,0], radius: 2.0, height: 1.0, axis: y)
          - cylinder(center: [0,0,0], radius: 0.5, height: 1.1, axis: y)

let gear = disk | for i in 0..24 {
    rotate(translate(tooth, z: 1.9), y: i * 15deg)
}

scene {
  camera { origin: [0,5,7], look_at: [0,0,0], up: [0,1,0], fov: 45deg }
  light  { origin: [5,10,5], color: [255,255,255] }
  object(gear, material: gold)
}
```

## Before writing any code

- Read the existing scene JSON schema exactly as the render engine expects it — `Material` fields, the CSG node shape (`{type: "csg", op, left, right}` — note this is **binary-only**; the DSL's n-ary infix syntax like `a | b | c` must lower to nested binary pairs to match), camera fields (including `fov_degrees`), light fields, object/mesh fields. This is the precise compilation target.
- Propose and document the CSG operator precedence table (e.g., does `-` bind tighter than `|`, matching how `*` binds tighter than `+`?) before implementing the parser — this is a real language-design decision, make it deliberately and write it down, don't let it fall out of implementation accident.

## Scope

Build as a new Rust crate (sibling to `hairy_tracer_core`, e.g. `scene_dsl/`), with these stages:

1. **Lexer** — tokenizes source into identifiers, keywords (`let`, `for`, `in`, `material`, `extends`, `scene`, `camera`, `light`, `object`), numeric literals (including unit-suffixed angle literals like `15deg`), punctuation/operators, string literals for names/paths
2. **Parser** — recursive descent for statements and blocks; Pratt parsing specifically for the CSG infix expression grammar, per the precedence table decided above
3. **AST** — expression nodes (CSG binary ops, primitive constructors, function calls with named arguments, identifier references, numeric/angle/vector literals) and statement nodes (`let`, `for`, `material`, `scene`/`object` declarations)
4. **Type checker** — enforces the `Angle`/`Scalar` distinction (reject mixing them), resolves identifier references (undefined variable = compile error, not a silent default), confirms `for` loop bounds are compile-time-constant integers
5. **Elaborator** — resolves `let`-bindings, unrolls `for` loops into concrete instances (bounds are compile-time constants per the total-language guarantee), resolves `material extends` by merging field maps (child overrides parent), fully evaluates CSG expressions down to concrete primitive trees with `Angle` values erased to radians
6. **JSON emitter** — walks the elaborated IR and emits the exact existing scene JSON schema, including lowering n-ary CSG expressions to the engine's binary-only node shape

A simple CLI entry point (`scene_dsl compile input.dsl -o output.json` or similar) that runs the full pipeline.

## Testing

Test each compiler stage directly, not just the end-to-end result — same discipline as the BVH/normal-interpolation/quaternion work earlier in this project, where testing the mechanism directly caught things a rendered image alone wouldn't have:

- **Lexer**: known input strings produce the expected token stream, specifically covering the `15deg` unit-suffix literal
- **Parser**: known source snippets produce the expected AST, specifically covering CSG operator precedence edge cases (e.g., confirm `a - b | c` parses with the documented precedence, not just "parses to something plausible")
- **Type checker**: confirm mixing `Angle` and `Scalar` produces a compile error, and confirm an undefined identifier produces a compile error — this is the direct test that the degrees/radians bug class is actually prevented, not just discouraged
- **Elaborator**: confirm `for i in 0..24 { ... }` unrolls to exactly 24 instances with correctly varying parameter values — this is the direct test against the original transform-collapse bug class, now made structurally hard to get wrong
- **Material inheritance**: confirm `extends` merges/overrides fields as expected
- **End-to-end**: rewrite the gear scene in the new DSL, compile it, and confirm the emitted JSON is functionally equivalent to the existing (fixed) `csg_gear.json`. Render it through the existing engine and confirm it produces a correct image — same quality as the already-fixed gear render.

This tool is purely additive (new crate, doesn't touch the render engine), so there's no regression risk to the existing render pipeline — no need to run the render-side snapshot suite for this task.

## Deliverable

- The `scene_dsl` crate: lexer, Pratt-based parser, AST, type checker, elaborator, JSON emitter, and CLI entry point
- The documented grammar and CSG operator precedence table
- The full per-stage test suite described above
- The gear scene rewritten in the new DSL (should be dramatically shorter than the ~800-line JSON version), compiled, and rendered — a concrete before/after demonstration of the win
