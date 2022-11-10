Week 2

`7 Nov 2022`

# Problem 1

Given two strings s1 and s2, where length(s1) < length(s2), find the index of
the first instance where s1 is a substring of s2.

```

Input: s1 = "abCd", s2 = "ffffabCdCefg" Output: 4
```

_Extension_: Return the index of all substrings of s1 in s2.

## Solution

An _amortized analysis_ of the problem will show that the best case complexity
of any solution to the problem is O(n), i.e. in linear time.

The idea for my solution is to have something like `char*` in C. However, unlike
in C, characters in Rust are Unicode scalar values that are internally stored in
memory as vectors. But the idea is pretty much the same.

So, given two strings `s1` and `s2`, s.t. `len(s1)` < `len(s2)` and we have to
find all instance of `s1` in `s2`.

```pseudo
p1 <- 0
len <- s2.len
FOR i <- 0:l
	IF ptr == l OR (ptr == l-1 AND s2[ptr] == s1[l-1])
		PRINT i-l
		ptr <- 0

	c <- s2[i]
	IF c == s1[ptr]
		ptr++
	ELSE IF c == s1[0]
		ptr <- 1
	ELSE
		ptr <- 0
```

```rust
let mut ptr = 0;

// O(n)
for (i, x) in s2.iter().enumerate() {
	let l = s1.len();

	// O(1)
	if ptr == l || (ptr == l - 1 && *x == s1[l - 1]) {
		println!("{}", i - l);
		ptr = 0;
	}

	// O(1)
	if let Some(c) = s1.get(ptr) {
		if c == x {
			ptr += 1;
		} else if *x == s1[0] {
			ptr = 1;
		} else {
			ptr = 0;
		}
	}
}
```

For the extension, I'll be using the same solution with first finding all
substrings of `s1`.

# Problem 2

You're on a game show and there are 3 doors, which are all shut.

The game show host asks you to pick a door. You pick a door and then the game
show host opens a door other than the one you picked, to reveal a pair of goats.
She says: "Behind one of the remaining doors is a goat and behind the other door
is a luxurious car". What is the best strategy to win the car?

1. Hold on to the door you selected originally
2. Switch and select the other door

Or, are the two strategies leading to the same result? Try writing a simulation
for this problem and report on the results :-)

# Problem 3

You want to sort an array of numbers. However, you have a problem. You are a
dictator and your philosophy is: if something doesn't follow your rules, kill
it! Let's introduce Stalin Sort:

1. You have an array of random positive integers.
2. In this array, if a number is "out of order" it shall be killed.
3. At the end of the algorithm, all numbers should appear consecutively. Any
   open spaces (if any) can only appear at the end of the array.

What's the fast implementation of Stalin Sort? Automatic weapons are not
allowed.

```
Input: [4, 17, 5, 6, 12, 1, 9]
Output: [4, 5, 6, 9]
```

Extension: Since cloning technology is not available, you must perform it
in-place. Stalin is merciful, and allows you to place a tombstone for each dead
number. Tombstones let you know a number was killed, and represent the empty
spaces at the end of the array. However, tombstones must be compliant with state
law on standardised cemetary supplies, so better make those tombstones the value
-1.

```
Input: [4, 17, 5, 6, 12, 1, 9]
Output: [4, 5, 6, 9, -1, -1, -1]
```
