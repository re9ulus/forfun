package main

import (
	"flag"
	"fmt"
	"io/ioutil"
)

func main() {
	filepath := flag.String("filepath", "", "Path to input file")
	flag.Parse()
	data, err := ioutil.ReadFile(*filepath)
	if err != nil {
		panic(err)
	}
	for i, j := 0, len(data)-1; i < j; i, j = i+1, j-1 {
		data[i], data[j] = data[j], data[i]
	}
	for i, v := range data {
		switch v {
		case 'A':
			data[i] = 'T'
		case 'T':
			data[i] = 'A'
		case 'C':
			data[i] = 'G'
		case 'G':
			data[i] = 'C'
		}
	}
	fmt.Println(string(data))
}
