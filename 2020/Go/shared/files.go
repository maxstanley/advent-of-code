package shared

import (
	"bufio"
	"os"
)

// ReadLinesFromFile reads each line from the file into a string array.
func ReadLinesFromFile(path string) ([]string, error) {
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
