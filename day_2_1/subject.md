
## --- Day 2: Dive! ---

Now, you need to figure out how to pilot this thing.

It seems like the submarine can take a series of commands like `forward 1`, `down 2`, or `up 3`:

* `forward X` increases the horizontal position by `X` units.
* `down X` *increases* the depth by `X` units.
* `up X` *decreases* the depth by `X` units.

<<<<<<< HEAD
Note that since you're on a submarine, `down` and `up` affect your *depth*, and so they have the opposite result of
what you might expect.

The submarine seems to already have a planned course (your puzzle input). You should probably figure out where it's
going. For example:
=======
Note that since you're on a submarine, `down` and `up` affect your *depth*, and so they have the opposite result of what you might expect.

The submarine seems to already have a planned course (your puzzle input). You should probably figure out where it's going. For example:
>>>>>>> 40734fd8fc8c6b9e1da735d347fc27daa7f55e3a

`forward 5
down 5
forward 8
up 3
down 8
forward 2
`

Your horizontal position and depth both start at `0`. The steps above would then modify them as follows:

* `forward 5` adds `5` to your horizontal position, a total of `5`.
* `down 5` adds `5` to your depth, resulting in a value of `5`.
* `forward 8` adds `8` to your horizontal position, a total of `13`.
* `up 3` decreases your depth by `3`, resulting in a value of `2`.
* `down 8` adds `8` to your depth, resulting in a value of `10`.
* `forward 2` adds `2` to your horizontal position, a total of `15`.

<<<<<<< HEAD
After following these instructions, you would have a horizontal position of `15` and a depth of `10`. (Multiplying
these together produces `*150*`.)

Calculate the horizontal position and depth you would have after following the planned course. *What do you get if you
multiply your final horizontal position by your final depth?*
=======
After following these instructions, you would have a horizontal position of `15` and a depth of `10`. (Multiplying these together produces `*150*`.)

Calculate the horizontal position and depth you would have after following the planned course. *What do you get if you multiply your final horizontal position by your final depth?*
>>>>>>> 40734fd8fc8c6b9e1da735d347fc27daa7f55e3a

To begin, [get your puzzle input][1].

Answer:

You can also [Shareon [Twitter][2] [Mastodon][3]] this puzzle.

[1] 2/input
<<<<<<< HEAD
[2] https://twitter.com/intent/tweet?text=%22Dive%21%22+%2D+Day+2+%2D+Advent+of+Code+2021&url=https%3A%2F%2Fadventofco
de%2Ecom%2F2021%2Fday%2F2&related=ericwastl&hashtags=AdventOfCode
=======
[2] https://twitter.com/intent/tweet?text=%22Dive%21%22+%2D+Day+2+%2D+Advent+of+Code+2021&url=https%3A%2F%2Fadventofcode%2Ecom%2F2021%2Fday%2F2&related=ericwastl&hashtags=AdventOfCode
>>>>>>> 40734fd8fc8c6b9e1da735d347fc27daa7f55e3a
[3] javascript:void(0);

