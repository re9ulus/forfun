package main

import (
	"bufio"
	"flag"
	"fmt"
	"os"
	"strings"
)

func main() {
	table := map[string]string{
		"UUU": "F",
		"CUU": "L",
		"AUU": "I",
		"GUU": "V",
		"UUC": "F",
		"CUC": "L",
		"AUC": "I",
		"GUC": "V",
		"UUA": "L",
		"CUA": "L",
		"AUA": "I",
		"GUA": "V",
		"UUG": "L",
		"CUG": "L",
		"AUG": "M",
		"GUG": "V",
		"UCU": "S",
		"CCU": "P",
		"ACU": "T",
		"GCU": "A",
		"UCC": "S",
		"CCC": "P",
		"ACC": "T",
		"GCC": "A",
		"UCA": "S",
		"CCA": "P",
		"ACA": "T",
		"GCA": "A",
		"UCG": "S",
		"CCG": "P",
		"ACG": "T",
		"GCG": "A",
		"UAU": "Y",
		"CAU": "H",
		"AAU": "N",
		"GAU": "D",
		"UAC": "Y",
		"CAC": "H",
		"AAC": "N",
		"GAC": "D",
		"UAA": "Stop",
		"CAA": "Q",
		"AAA": "K",
		"GAA": "E",
		"UAG": "Stop",
		"CAG": "Q",
		"AAG": "K",
		"GAG": "E",
		"UGU": "C",
		"CGU": "R",
		"AGU": "S",
		"GGU": "G",
		"UGC": "C",
		"CGC": "R",
		"AGC": "S",
		"GGC": "G",
		"UGA": "Stop",
		"CGA": "R",
		"AGA": "R",
		"GGA": "G",
		"UGG": "W",
		"CGG": "R",
		"AGG": "R",
		"GGG": "G",
	}
	filename := flag.String("filename", "", "input filename")
	flag.Parse()
	file, err := os.Open(*filename)
	if err != nil {
		panic(err)
	}
	scanner := bufio.NewScanner(file)
	scanner.Split(bufio.ScanLines)
	scanner.Scan()
	line := scanner.Bytes()
	var result strings.Builder
	var buffer []byte
	for _, ch := range line {
		buffer = append(buffer, ch)
		if len(buffer) == 3 {
			it := table[string(buffer)]
			if it == "Stop" {
				break
			}
			result.WriteString(it)
			buffer = buffer[:0]
		}
	}
	fmt.Println(result.String())
}
