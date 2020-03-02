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
	for i, v := range data {
		if v == 'T' {
			data[i] = 'U'
		}
	}
	fmt.Println(string(data))
}
