package main

import (
	"flag"
	"fmt"
	"io/ioutil"
)

func main() {
	filename := flag.String("filename", "", "Path to input file")
	flag.Parse()

	data, err := ioutil.ReadFile(*filename)
	if err != nil {
		panic(err)
	}

	counters := make(map[uint8]int)
	for _, chr := range data {
		counters[chr]++
	}

	delim := ""
	for _, chr := range []byte("ACGT") {
		fmt.Print(delim)
		fmt.Print(counters[chr])
		delim = " "
	}
}
