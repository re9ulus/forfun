package main

import (
	"bufio"
	"flag"
	"fmt"
	"os"
)

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func zFunc(s string) []int {
	z := make([]int, len(s))
	left, right := 0, 0
	for i := 1; i < len(s); i++ {
		if i <= right {
			z[i] = min(right-i+1, z[i-left])
		}
		for i+z[i] < len(s) && s[z[i]] == s[i+z[i]] {
			z[i]++
		}
		if i+z[i]-1 > right {
			left = i
			right = i + z[i] - 1
		}
	}
	return z
}

func getPatternLocations(text, pattern string) {
	zf := zFunc(pattern + "$" + text)
	delim := ""
	for i := len(pattern) + 1; i < len(zf); i++ {
		if zf[i] == len(pattern) {
			fmt.Print(delim)
			fmt.Print(i - len(pattern))
			delim = " "
		}
	}
}

func main() {
	filename := flag.String("filename", "", "input filename")
	flag.Parse()
	file, err := os.Open(*filename)
	if err != nil {
		panic(err)
	}
	scanner := bufio.NewScanner(file)
	scanner.Split(bufio.ScanLines)
	scanner.Scan()
	text := scanner.Text()
	scanner.Scan()
	pattern := scanner.Text()
	getPatternLocations(text, pattern)
}
