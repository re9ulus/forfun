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
	for _, val := range buffer {
		fmt.Print(delim)
		fmt.Print(val)
		delim = " "
	}
	fmt.Println()
}

func getNextPermutation(buffer []int, values []int, used []bool) {
	if len(buffer) == len(used) {
		printBuffer(buffer)
		return
	}
	for i := range used {
		if !used[i] {
			buffer = append(buffer, values[i])
			used[i] = true
			getNextPermutation(buffer, values, used)
			used[i] = false
			buffer = buffer[:len(buffer)-1]
		}
	}
}

func generatePermutations(val int) {
	values := make([]int, val)
	used := make([]bool, val)
	for i := 0; i < val; i++ {
		values[i] = i + 1
	}
	var buffer []int
	getNextPermutation(buffer, values, used)
}

func factorial(n int) int {
	ans := 1
	for i := 2; i <= n; i++ {
		ans *= i
	}
	return ans
}

func main() {
	filename := flag.String("filename", "", "input filename")
	flag.Parse()
	file, err := os.Open(*filename)
	if err != nil {
		panic(err)
	}
	defer file.Close()
	scanner := bufio.NewScanner(file)
	scanner.Scan()
	val, err := strconv.Atoi(scanner.Text())
	if err != nil {
		panic(err)
	}
	fmt.Println(factorial(val))
	generatePermutations(val)
}
