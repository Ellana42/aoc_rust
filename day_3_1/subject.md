
## --- Day 3: Binary Diagnostic ---

The submarine has been making some odd creaking noises, so you ask it to produce a diagnostic report just in case.

<<<<<<< HEAD
The diagnostic report (your puzzle input) consists of a list of binary numbers which, when decoded properly, can tell you many useful things about the conditions of the submarine. The first parameter to check is the *power
consumption*.

You need to use the binary numbers in the diagnostic report to generate two new binary numbers (called the *gamma rate* and the *epsilon rate*). The power consumption can then be found by multiplying the gamma rate by the epsilon rate.

Each bit in the gamma rate can be determined by finding the *most common bit in the corresponding position* of all numbers in the diagnostic report. For example, given the following diagnostic report:
=======
The diagnostic report (your puzzle input) consists of a list of binary numbers which, when decoded properly, can tell
you many useful things about the conditions of the submarine. The first parameter to check is the *power consumption*.

You need to use the binary numbers in the diagnostic report to generate two new binary numbers (called the *gamma
rate* and the *epsilon rate*). The power consumption can then be found by multiplying the gamma rate by the epsilon
rate.

Each bit in the gamma rate can be determined by finding the *most common bit in the corresponding position* of all
numbers in the diagnostic report. For example, given the following diagnostic report:
>>>>>>> 40734fd8fc8c6b9e1da735d347fc27daa7f55e3a

`00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
`

<<<<<<< HEAD
Considering only the first bit of each number, there are five `0` bits and seven `1` bits. Since the most common bit is `1`, the first bit of the gamma rate is `1`.

The most common second bit of the numbers in the diagnostic report is `0`, so the second bit of the gamma rate is `0`.

The most common value of the third, fourth, and fifth bits are `1`, `1`, and `0`, respectively, and so the final three bits of the gamma rate are `110`.

So, the gamma rate is the binary number `10110`, or `*22*` in decimal.

The epsilon rate is calculated in a similar way; rather than use the most common bit, the least common bit from each position is used. So, the epsilon rate is `01001`, or `*9*` in decimal. Multiplying the gamma rate (`22`) by the
epsilon rate (`9`) produces the power consumption, `*198*`.

Use the binary numbers in your diagnostic report to calculate the gamma rate and epsilon rate, then multiply them together. *What is the power consumption of the submarine?* (Be sure to represent your answer in decimal, not binary.)
=======
Considering only the first bit of each number, there are five `0` bits and seven `1` bits. Since the most common bit
is `1`, the first bit of the gamma rate is `1`.

The most common second bit of the numbers in the diagnostic report is `0`, so the second bit of the gamma rate is `0`.

The most common value of the third, fourth, and fifth bits are `1`, `1`, and `0`, respectively, and so the final three
bits of the gamma rate are `110`.

So, the gamma rate is the binary number `10110`, or `*22*` in decimal.

The epsilon rate is calculated in a similar way; rather than use the most common bit, the least common bit from each
position is used. So, the epsilon rate is `01001`, or `*9*` in decimal. Multiplying the gamma rate (`22`) by the
epsilon rate (`9`) produces the power consumption, `*198*`.

Use the binary numbers in your diagnostic report to calculate the gamma rate and epsilon rate, then multiply them
together. *What is the power consumption of the submarine?* (Be sure to represent your answer in decimal, not binary.)
>>>>>>> 40734fd8fc8c6b9e1da735d347fc27daa7f55e3a

To begin, [get your puzzle input][1].

Answer:

You can also [Shareon [Twitter][2] [Mastodon][3]] this puzzle.

[1] 3/input
<<<<<<< HEAD
[2] https://twitter.com/intent/tweet?text=%22Binary+Diagnostic%22+%2D+Day+3+%2D+Advent+of+Code+2021&url=https%3A%2F%2Fadventofcode%2Ecom%2F2021%2Fday%2F3&related=ericwastl&hashtags=AdventOfCode
=======
[2] https://twitter.com/intent/tweet?text=%22Binary+Diagnostic%22+%2D+Day+3+%2D+Advent+of+Code+2021&url=https%3A%2F%2F
adventofcode%2Ecom%2F2021%2Fday%2F3&related=ericwastl&hashtags=AdventOfCode
>>>>>>> 40734fd8fc8c6b9e1da735d347fc27daa7f55e3a
[3] javascript:void(0);

