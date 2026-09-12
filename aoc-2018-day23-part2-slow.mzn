% I wrote this, and I'm proud of this, but it's too slow for the real input.

% These parameters need to come from a DZN data file.
int: n;
array[1..n, 1..3] of int: nanobots;
array[1..n] of int: ranges;

int: min_x = min(nanobots[.., 1]);
int: min_y = min(nanobots[.., 2]);
int: min_z = min(nanobots[.., 3]);

int: max_x = max(nanobots[.., 1]);
int: max_y = max(nanobots[.., 2]);
int: max_z = max(nanobots[.., 3]);

var min_x..max_x: x;
var min_y..max_y: y;
var min_z..max_z: z;

var 0..n: in_range;

constraint in_range = sum(j in 1..n)(bool2int(
  abs(x - nanobots[j,1]) +
  abs(y - nanobots[j,2]) +
  abs(z - nanobots[j,3]) <= ranges[j]
));

solve maximize in_range;
