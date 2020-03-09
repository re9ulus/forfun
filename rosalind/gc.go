package main

import (
	"bufio"
	"flag"
	"fmt"
	"os"
)

type FASTARec struct {
	Label string
	Data  []byte
}

func computeGC(data []byte) float64 {
	gcCount := 0
	for _, ch := range data {
		if ch == 'G' || ch == 'C' {
			gcCount++
		}
	}
	return float64(gcCount) / float64(len(data)) * 100.0
}

func findMaxGC(recs []FASTARec) (FASTARec, float64) {
	maxGC := 0.0
	maxId := -1
	for i, rec := range recs {
		gc := computeGC(rec.Data)
		if gc > maxGC {
			maxGC = gc
			maxId = i
		}
	}
	return recs[maxId], maxGC
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
	var records []FASTARec
	var buffer []byte
	var label string
	for scanner.Scan() {
		line := scanner.Text()
		if line[0] == '>' {
			if len(buffer) > 0 {
				records = append(records, FASTARec{
					Label: label,
					Data:  buffer,
				})
			}
			buffer = nil
			label = line[1:]
			continue
		}
		buffer = append(buffer, []byte(line)...)
	}
	if len(buffer) > 0 {
		records = append(records, FASTARec{
			Label: label,
			Data:  buffer,
		})
	}
	rec, gc := findMaxGC(records)
	fmt.Println(rec.Label)
	fmt.Println(gc)
}
