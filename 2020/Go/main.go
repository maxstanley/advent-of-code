package main

import (
	. "aoc/day01"
	. "aoc/day02"
	. "aoc/day03"
	. "aoc/day04"
	"fmt"
)

func main() {
	challenges := []func(){
		Day01,
		Day02,
		Day03,
		Day04,
	}

	lineBreak()
	for _, challenge := range challenges {
		challenge()
		lineBreak()
	}
}

func lineBreak() {
	fmt.Println("======")
}
