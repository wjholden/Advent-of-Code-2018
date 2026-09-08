%% I had to take some help from Gemini for this one.
%%
%% The basic idea (commented out below) works for the sample input,
%% but it's too slow for the real problem. Gemini came up with an
%% idea to use this "Big-M" technique that I had never heard of.
%% You need to run this with COIN-BC.
%%
%% I tried to port the idea over to Z3 and, surprisingly, it didn't
%% work until I went back to the more straightforward idea with
%% absolute values shown here. Maybe someday I'll go back and make
%% this work with Pumpkin.

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

% var 0..n: in_range;

% constraint in_range = sum(j in 1..n)(bool2int(
%   abs(x - nanobots[j,1]) +
%   abs(y - nanobots[j,2]) +
%   abs(z - nanobots[j,3]) <= ranges[j]
% ));

% solve maximize in_range;


% Binary indicators: 1 if nanobot j is in range, 0 otherwise
array[1..n] of var 0..1: in_range;

% Big-M constant: Must be larger than any maximum possible distance in the bounding box
int: M = (max_x - min_x) + (max_y - min_y) + (max_z - min_z) + max(ranges);

% Unroll the absolute values into 8 linear inequalities per nanobot
constraint forall(j in 1..n)(
  let {
    int: px = nanobots[j,1],
    int: py = nanobots[j,2],
    int: pz = nanobots[j,3],
    int: r  = ranges[j]
  } in (
    ( x - px) + ( y - py) + ( z - pz) <= r + M * (1 - in_range[j]) /\
    ( x - px) + ( y - py) - ( z - pz) <= r + M * (1 - in_range[j]) /\
    ( x - px) - ( y - py) + ( z - pz) <= r + M * (1 - in_range[j]) /\
    ( x - px) - ( y - py) - ( z - pz) <= r + M * (1 - in_range[j]) /\
    (-x + px) + ( y - py) + ( z - pz) <= r + M * (1 - in_range[j]) /\
    (-x + px) + ( y - py) - ( z - pz) <= r + M * (1 - in_range[j]) /\
    (-x + px) - ( y - py) + ( z - pz) <= r + M * (1 - in_range[j]) /\
    (-x + px) - ( y - py) - ( z - pz) <= r + M * (1 - in_range[j])
  )
);

% Maximize the number of nanobots in range
solve maximize sum(in_range);
var int: solution;
constraint solution = abs(x) + abs(y) + abs(z);