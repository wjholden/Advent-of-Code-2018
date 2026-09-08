int: n = 9;

array[1..n, 1..3] of int: nanobots =
[|0,0,0,
 |1,0,0,
 |4,0,0,
 |0,2,0,
 |0,5,0,
 |0,0,3,
 |1,1,1,
 |1,1,2,
 |1,3,1,
 |];

array[1..n] of int: ranges = [4,1,3,1,3,1,1,1,1];

var int: i; % index of the strongest nanobot

var int: in_range;

constraint in_range = sum(j in 1..n)(bool2int(
  abs(nanobots[i,1] - nanobots[j,1]) +
  abs(nanobots[i,2] - nanobots[j,2]) +
  abs(nanobots[i,3] - nanobots[j,3]) <= ranges[i]
));

solve maximize ranges[i];