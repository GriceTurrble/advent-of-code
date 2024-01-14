# Advent of Code 2023 Day 9

Link: <https://adventofcode.com/2023/day/9>

## Part 1

I realized pretty quick on this one that a recursive solution would be easiest to achieve:

- Take a set of numbers as an input
- If those numbers are all 0, return 0
- Otherwise, produce the next set of numbers as a Vector by iterating through the input over the range of indices `0..length-1` (one less than the final index), pushing `nums[i+1] - nums[i]` into the new set.
- Finally, recurse with this new set of numbers, adding the result of the recursion to `nums[len-1]` (the final number in the input).

Do this for every line of input, total things up, and we have an answer.

## Part 2

This was pretty easy. I copied over the earlier algo, only changing the name of the func and doing `nums[0] - <recursive>` instead of adding to the end. Done deal!

I only completed this several hours after the first part due to heading out for a party. Saturdays with the family, and all. :)
