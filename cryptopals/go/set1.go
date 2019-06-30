package main

import (
    "fmt"
)


// Convert hex to base64
func task1(isVerbose bool) {
    fmt.Println("Task 1...")
    hexInput := "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"
    base64Expected := "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"

    base64Real := HexToBase64(hexInput)
    if base64Expected != base64Real {
        panic("Set 1 Tasl 1 failed")
    }
    if isVerbose {
        fmt.Println(base64Real)
    }
    fmt.Println("Task 1 Done.")
}


// Fixed xor
func task2(isVerbose bool) {
    fmt.Println("Task 2...")
    a := "1c0111001f010100061a024b53535009181c"
    b := "686974207468652062756c6c277320657965"
    expected := "746865206b696420646f6e277420706c6179"
    actual := XorStrings(a, b)
    fmt.Println(actual)
    if actual != expected {
        panic("Set 2 Task 2 failed")
    }
    fmt.Println("Task 2 Done.")
}


func main() {
    isVerbose := true
    fmt.Println("Set 1...")
    task1(isVerbose)
    task2(isVerbose)
    fmt.Println("Set 1 Done.")
}
