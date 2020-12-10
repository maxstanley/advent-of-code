package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

/*
After saving Christmas five years in a row, you've decided
to take a vacation at a nice resort on a tropical island.
Surely, Christmas will go on without you.

The tropical island has its own currency and is entirely
cash-only. The gold coins used there have a little picture
of a starfish; the locals just call them stars. None of the
currency exchanges seem to have heard of them, but somehow,
you'll need to find fifty of these coins by the time you
arrive so you can pay the deposit on your room.

To save your vacation, you need to get all fifty stars by
December 25th.

Collect stars by solving puzzles. Two puzzles will be made
available on each day in the Advent calendar; the second
puzzle is unlocked when you complete the first. Each puzzle
grants one star. Good luck!

Before you leave, the Elves in accounting just need you to
fix your expense report (your puzzle input); apparently,
something isn't quite adding up.

Specifically, they need you to find the two entries that sum
to 2020 and then multiply those two numbers together.

For example, suppose your expense report contained the
following:
1721
979
366
299
675
1456

In this list, the two entries that sum to 2020 are 1721 and
299. Multiplying them together produces 1721 * 299 = 514579,
so the correct answer is 514579.

Of course, your expense report is much larger. Find the two
entries that sum to 2020; what do you get if you multiply
them together? 365619

--- Part Two ---
The Elves in accounting are thankful for your help; one of
them even offers you a starfish coin they had left over from
a past vacation. They offer you a second one if you can find
three numbers in your expense report that meet the same
criteria.

Using the above example again, the three entries that sum to
2020 are 979, 366, and 675. Multiplying them together
produces the answer, 241861950.

In your expense report, what is the product of the three
entries that sum to 2020? 236873508
*/

func main() {
	fmt.Println("Day 01")

	lines, _ := readLinesFromFile("./input.txt")
	numbers := stringArrayToIntegerArray(lines)
	a, b := getSum2Items(numbers, 2020)
	x, y, z := getSum3Items(numbers, 2020)

	fmt.Printf("%d * %d = %d\n", a, b, a*b)
	fmt.Printf("%d * %d * %d = %d\n", x, y, z, x*y*z)
}

// #part-1
func getSum2Items(numbers []int, target int) (int, int) {
	for i := 0; i < len(numbers); i++ {
		for j := i + 1; j < len(numbers); j++ {
			if target-numbers[i]-numbers[j] == 0 {
				return numbers[i], numbers[j]
			}
		}
	}

	return -1, -1
}

// #part-1
func getSum3Items(numbers []int, target int) (int, int, int) {
	for i := 0; i < len(numbers); i++ {
		for j := i + 1; j < len(numbers); j++ {
			for k := i + 2; k < len(numbers); k++ {
				if target-numbers[i]-numbers[j]-numbers[k] == 0 {
					return numbers[i], numbers[j], numbers[k]
				}

			}
		}
	}

	return -1, -1, -1
}

func stringArrayToIntegerArray(sArray []string) (iArray []int) {
	for _, s := range sArray {
		i, _ := strconv.Atoi(s)
		iArray = append(iArray, i)
	}
	return
}

func readLinesFromFile(path string) ([]string, error) {
	file, err := os.Open(path)

	if err != nil {
		return nil, err
	}
	defer file.Close()

	var lines []string
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}
	return lines, scanner.Err()
}
