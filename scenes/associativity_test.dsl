let A = cube(min: [-1.0, -1.0, -1.0], max: [1.0, 1.0, 1.0])
let B = cube(min: [0.0, -1.0, -1.0], max: [2.0, 1.0, 1.0])
let C = cube(min: [0.5, -1.0, -1.0], max: [2.5, 1.0, 1.0])

let left_assoc = A - B - C
let right_assoc = A - (B - C)

scene {
  object(left_assoc)
  object(right_assoc)
}
