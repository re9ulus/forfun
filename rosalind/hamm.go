package main

import (
	"bufio"
	"flag"
	"fmt"
	"os"
)

func readLine(s *bufio.Scanner) (string, bool) {
	if s.Scan() {
		return s.Text(), true
	}
	return "", false
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
	line1, _ := readLine(scanner)
	line2, _ := readLine(scanner)

	diff := 0
	for i := range line1 {
		if line1[i] != line2[i] {
			diff++
		}
	}
	fmt.Println(diff)
}
