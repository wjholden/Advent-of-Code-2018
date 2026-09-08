import re

from z3 import Abs, If, Int, Optimize, Sum, sat

#input_file = "samples/day23-2.txt"
input_file = "puzzles/day23.txt"
with open(input_file) as file:
    content = file.read()
xyzr = re.findall(r'pos=<(?P<x>-?\d+),(?P<y>-?\d+),(?P<z>-?\d+)>, r=(?P<r>\d+)', content)
xyzr = [tuple(int(i) for i in t) for t in xyzr]
x = [t[0] for t in xyzr]
y = [t[1] for t in xyzr]
z = [t[2] for t in xyzr]
r = [t[3] for t in xyzr]
n = len(xyzr)

X = Int('X')
Y = Int('Y')
Z = Int('Z')
#in_range = [Int(f"{i} in range") for i in range(n)]

# # Big M method too slow on Z3. This works on the small input.
# M = (max(x) - min(x)) + (max(y) - min(y)) + (max(z) - min(z)) + max(r)

# c1 = [Or(r == 0, r == 1) for r in in_range]

# pm = list(product({-1,1},{-1,1},{-1,1}))
# c2 = [
#     pmx * (X - x[j]) + pmy * (Y - y[j]) + pmz * (Z - z[j]) <= r[j] + M * (1 - in_range[j])
#     for (pmx, pmy, pmz) in pm
#     for j in range(n)
# ]

# opt = Optimize()
# opt.add(c1 + c2)
# opt.maximize(Sum(in_range))
# opt.check()
# opt.model()

in_range = []
for i in range(n):
    in_range.append(If(Sum(Abs(X - x[i]), Abs(Y - y[i]), Abs(Z - z[i])) <= r[i], 1, 0))

solution = Int('solution')

opt = Optimize()
opt.add(solution == X + Y + Z)
opt.maximize(Sum(in_range))
if opt.check() == sat:
    m = opt.model()
    print(f"Part 2: {m[solution]}")
