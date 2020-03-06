package main

import (
	"bufio"
	"flag"
	"fmt"
	"os"
	"sort"
	"strconv"
)

func printBuffer(buffer []string) {
	for _, item := range buffer {
		fmt.Print(item)
	}
	fmt.Println()
}

func getNextCombination(buffer, items []string, n int) {
	if len(buffer) == n {
		printBuffer(buffer)
		return
	}
	for _, item := range items {
		buffer = append(buffer, item)
		getNextCombination(buffer, items, n)
		buffer = buffer[:len(buffer)-1]
	}
}

func getCombinations(items []string, n int) {
	var buffer []string
	getNextCombination(buffer, items, n)
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
	scanner.Split(bufio.ScanWords)
	var items []string
	for scanner.Scan() {
		items = append(items, scanner.Text())
	}
	n, err := strconv.Atoi(items[len(items)-1])
	if err != nil {
		panic(err)
	}
	items = items[:len(items)-1]
	sort.Strings(items)
	getCombinations(items, n)
}
