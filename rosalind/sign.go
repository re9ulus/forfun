package main

import (
	"bufio"
	"flag"
	"fmt"
	"os"
	"strconv"
)

func printBuffer(buffer []int) {
	delim := ""
	for i := 0; i < len(buffer); i++ {
		fmt.Print(delim)
		fmt.Print(buffer[i])
		delim = " "
	}
	fmt.Println()
}

func getNextPermutation(buffer []int, items []int, used []bool) {
	if len(buffer) == len(used) {
		printBuffer(buffer)
		return
	}
	for i := 0; i < len(used); i++ {
		if !used[i] {
			used[i] = true
			buffer = append(buffer, items[i])
			getNextPermutation(buffer, items, used)
			buffer[len(buffer)-1] *= -1
			getNextPermutation(buffer, items, used)
			buffer = buffer[:len(buffer)-1]
			used[i] = false
		}
	}
}

func generatePermutations(n int) {
	items := make([]int, n)
	used := make([]bool, n)
	var buffer []int
	for i := 0; i < n; i++ {
		items[i] = i + 1
	}
	getNextPermutation(buffer, items, used)
}

func main() {
	counters := map[int]int{
		1: 2,
		2: 8,
		3: 48,
		4: 384,
		5: 3840,
		6: 46080,
	}
	filename := flag.String("filename", "", "input filename")
	flag.Parse()
	file, err := os.Open(*filename)
	if err != nil {
		panic(err)
	}
	defer file.Close()
	scanner := bufio.NewScanner(file)
	scanner.Scan()
	n, err := strconv.Atoi(scanner.Text())
	if err != nil {
		panic(err)
	}
	fmt.Println(counters[n])
	generatePermutations(n)
}
