# My results on submission
| Day | Part 1   | Part 2   |   Notes    |
|-----|----------|----------|------------|
| 1   |✅|✅|None|
| 2   |✅|❌✅|Faulty method failed to remove invalid gaps in first or last position|
| 3   |✅|❌❌✅ |Regex for don't regions failed to account for newline characters and final dont region at end of string|
| 4   |❌✅|✅| Max index Error. Missed XMAS where S was in last row or col and X->M->A->S moved towards the edge (not in test input)|
| 5   |✅|✅|None|
| 6   |✅|❌✅|After identifying potential obstacle, didn't start guard from initial position, started from position that would bump obstacle. Neglected obstacle could have prevented guard from reaching curr state|
| 7   |❌✅|✅| Faulty continue clause when checking if grew larger than target.|
| 8   |✅|✅|Thought three Ts text box was a second input and spent time debugging it (it wasn't an input)|
| 9   |✅|✅| None |
| 10  |✅|✅| Solved Part 2 by accident when attempting part 1, part 2 faster?|
| 11  |❌✅|✅| Forgot to account for splits producing 0s on the RHS. Then the num str needs be length 1. Needed to completely reformulate to reduce operations to unique numbers instead of calculating stone by stone|
| 12  |✅|❌✅| Missed a single 1 cell internal border because it was surrounded by the external border of another group. Soooo much debugging. Count the corners next time instead of following the borders.... corners = sides.|
| 13  |❌✅|❌✅|Ignored Floating Point Error requiring round up from .999 in part 1. In part 2, realized floating point error filter condition was too strict, reduced it from 5 decimal places to 3|
| 14  ||||
| 15  ||||
| 16  ||||
| 17  ||||
| 18  ||||
| 19  ||||
| 20  ||||
| 21  ||||
| 22  ||||
| 23  ||||
| 24  ||||
| 25  ||||
||||
