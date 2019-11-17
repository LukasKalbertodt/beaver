# Results

## 2019-11-17

#### N=1

```
▸ Analyzing all 36 TMs with 1 states...

36 / 36 [====================================================================================================================================================================================] 100.00 % 39610.28/s

  (That took 935.65µs, 25.99µs per TM on 8 threads -> 207.921µs core time per TM)

▸ Results:
- The high score (number of 1s after halting) is: 1
  - 6 TMs reached that high score
  - The quickest of which reached the high score in 1 steps
- 6 (16.67%) TMs halted but did not get a high score
  - 12 (33.33%) TMs halted after 1 step (their first transition was to the halt state)
- 24 (66.67%) did not terminate:
  - 24 (66.67%) immediately ran away in one direction and remained in the start state
  - 0 (0.00%) did not contain a transition to the halt state
  - 0 (0.00%) statically could not reach the halt state
  - 0 (0.00%) were caught in a run-away loop
  - 0 (0.00%) were aborted after the maximum number of steps (200)


   (histogram not shown as TMs ran for at most 1 step)

```

#### N=2

```
▸ Analyzing all 10000 TMs with 2 states...

10000 / 10000 [============================================================================================================================================================================] 100.00 % 9117856.50/s

  (That took 1.12ms, 112ns per TM on 8 threads -> 898ns core time per TM)

▸ Results:
- The high score (number of 1s after halting) is: 4
  - 2 TMs reached that high score
  - The quickest of which reached the high score in 6 steps
- 3042 (30.42%) TMs halted but did not get a high score
  - 2000 (20.00%) TMs halted after 1 step (their first transition was to the halt state)
- 6956 (69.56%) did not terminate:
  - 4000 (40.00%) immediately ran away in one direction and remained in the start state
  - 2048 (20.48%) did not contain a transition to the halt state
  - 288 (2.88%) statically could not reach the halt state
  - 528 (5.28%) were caught in a run-away loop
  - 92 (0.92%) were aborted after the maximum number of steps (200)


▸ Histogram (how many TMs halted after x steps):
note: the y-axis is logarithmic

      800 ▕ ██
          ▕ ██
      210 ▕ ██
          ▕ ██ ▃▃
       86 ▕ ██ ██
          ▕ ██ ██ ▁▁
       35 ▕ ██ ██ ██
          ▕ ██ ██ ██
       14 ▕ ██ ██ ██    ▆▆
          ▕ ██ ██ ██    ██
        6 ▕ ██ ██ ██ ▆▆ ██
          ▕ ██ ██ ██ ██ ██
        2 ▕ ██ ██ ██ ██ ██
          ▕ ██ ██ ██ ██ ██
        0 ▕▁██▁██▁██▁██▁██▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁
    steps:   2  3  4  5  6  7  8  9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29
    count:        56  8 20  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0

```

#### N=3

```
▸ Analyzing all 7529536 TMs with 3 states...

7530000 / 7529536  100.01 % 53684609.68/s

  (That took 140.28ms, 18ns per TM on 8 threads -> 149ns core time per TM)

▸ Results:
- The high score (number of 1s after halting) is: 6
  - 20 TMs reached that high score
  - The quickest of which reached the high score in 11 steps
- 2147164 (28.51%) TMs halted but did not get a high score
  - 1075648 (14.28%) TMs halted after 1 step (their first transition was to the halt state)
- 5382816 (71.48%) did not terminate:
  - 2151760 (28.58%) immediately ran away in one direction and remained in the start state
  - 1990656 (26.44%) did not contain a transition to the halt state
  - 491392 (6.53%) statically could not reach the halt state
  - 568416 (7.55%) were caught in a run-away loop
  - 180592 (2.40%) were aborted after the maximum number of steps (200)


▸ Histogram (how many TMs halted after x steps):
note: the y-axis is logarithmic

   614656 ▕ ██
          ▕ ██ ██
    42747 ▕ ██ ██ ██ ▂▂
          ▕ ██ ██ ██ ██ ▂▂
     7229 ▕ ██ ██ ██ ██ ██ ▅▅
          ▕ ██ ██ ██ ██ ██ ██ ▄▄
     1223 ▕ ██ ██ ██ ██ ██ ██ ██ ▇▇
          ▕ ██ ██ ██ ██ ██ ██ ██ ██ ▇▇ ▁▁
      207 ▕ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ▄▄
          ▕ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ▇▇ ▄▄       ▄▄
       35 ▕ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ▂▂    ██
          ▕ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ▂▂ ▂▂
        6 ▕ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ▄▄
          ▕ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██
        0 ▕▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁
    steps:   2  3  4  5  6  7  8  9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29
    count:                                         40 32    32 16 16  8  0  0  0  0  0  0  0  0

```

#### N=4

```
▸ Analyzing all 11019960576 TMs with 4 states...

11020000000 / 11019960576  100.00 % 36328019.19/s

  (That took 303.35s, 27ns per TM on 8 threads -> 220ns core time per TM)

▸ Results:
- The high score (number of 1s after halting) is: 13
  - 24 TMs reached that high score
  - The quickest of which reached the high score in 96 steps
- 2985384456 (27.09%) TMs halted but did not get a high score
  - 1224440064 (11.11%) TMs halted after 1 step (their first transition was to the halt state)
- 8034615520 (72.91%) did not terminate:
  - 2448919552 (22.22%) immediately ran away in one direction and remained in the start state
  - 3221225472 (29.23%) did not contain a transition to the halt state
  - 962875392 (8.74%) statically could not reach the halt state
  - 970814256 (8.81%) were caught in a run-away loop
  - 430780848 (3.91%) were aborted after the maximum number of steps (200)


▸ Histogram (how many TMs halted after x steps):
note: the y-axis is logarithmic

816293376 ▕ ██ ▅▅
          ▕ ██ ██ ██ ▅▅ ▁▁
 13473394 ▕ ██ ██ ██ ██ ██ ▆▆ ▂▂
          ▕ ██ ██ ██ ██ ██ ██ ██ ▆▆ ▄▄
   873436 ▕ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ▅▅ ▂▂
          ▕ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ▅▅ ▂▂ ▃▃ ▁▁
    56622 ▕ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ▅▅ ▄▄ ▂▂
          ▕ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ▇▇ ▅▅ ▃▃ ▂▂ ▁▁
     3671 ▕ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ▆▆ ▅▅ ▅▅ ▅▅ ▁▁ ▂▂ ▁▁
          ▕ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ▅▅ ▃▃ ▁▁ ▄▄ ▂▂
      238 ▕ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ▆▆ ▇▇ ▆▆ ▅▅ ▄▄ ▃▃ ▂▂ ▂▂ ▂▂    ▁▁ ▂▂
          ▕ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ▇▇ ██ ██ ▆▆ ▂▂ ▅▅ ▇▇ ▆▆    ▂▂ ▂▂ ▂▂ ▃▃ ▂▂ ▅▅          ▅▅ ▃▃
       15 ▕ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ▇▇ ██ ██ ██ ██ ██ ██ ▇▇ ▃▃ ▇▇ ██ ██ ▃▃ ▃▃       ▃▃ ▃▃       ▇▇             ▃▃ ▃▃                                  ▇▇ ▃▃                            ▃▃
          ▕ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██ ██       ██ ██       ██             ██ ██                                  ██ ██                            ██
        0 ▕▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁██▁▁▁▁▁▁▁██▁██▁▁▁▁▁▁▁██▁▁▁▁▁▁▁▁▁▁▁▁▁██▁██▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁██▁██▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁██
    steps:   2  3  4  5  6  7  8  9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 50 51 52 53 54 55 56 57 58 59 60 61 62 63 64 65 66 67 68 69 70 71 72 73 74 75 76 77 78 79 80 81 82 83 84 85 86 87 88 89 90 91 92 93 94 95 96 97 98 99100101102103104105106107
    count:                                                                                                                                                              72          48 72 72 72 96 72    48 24 48    96 24 24  0  0 24 24  0  0 48  0  0  0  0 24 24  0  0  0  0  0  0  0  0  0  0  0 48 24  0  0  0  0  0  0  0  0  0 24


```
