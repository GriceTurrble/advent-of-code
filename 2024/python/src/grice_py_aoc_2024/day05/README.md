# Day 5

In hindsight of this day, I note there are easier ways to determine if a match is correct.
Instead of popping things from a queue and checking membership in a set,
I could have used a sliding window and tested to see if a given 2-tuple set of pages
existed as a rule in the correct order.

That insight lead me to implement the sorter and `functools.cmp_to_key`,
which I augmented with `functools.partial` to inject the `rules_dict` I had so painstakingly built,
thus making `sorted(..., key=my_sorter)` a possibility.

Worked it out at the end, though, and finished off in record time for my usual solutions. :)
