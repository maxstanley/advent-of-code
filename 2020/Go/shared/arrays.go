package shared

import (
	"strconv"
	"strings"
)

// StringArrayToIntegerArray converts an array of strings to an array of integers.
func StringArrayToIntegerArray(sArray []string) (iArray []int) {
	for _, s := range sArray {
		i, _ := strconv.Atoi(s)
		iArray = append(iArray, i)
	}
	return
}

// KeyValueSplit splits a string into a key/value pair based on a delimeter.
func KeyValueSplit(items []string, splitChar string) (pairs [][]string) {
	for _, item := range items {
		pair := strings.Split(item, splitChar)
		pairs = append(pairs, pair)
	}
	return
}
