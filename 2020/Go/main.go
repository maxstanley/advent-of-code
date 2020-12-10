package main

import (
	. "aoc/day01"
	. "aoc/day02"
	"fmt"
)

func main() {
	challenges := []func(){
		Day01,
		Day02,
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