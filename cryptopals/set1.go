package main

import (
    "fmt"
)


func task1(isVerbose bool) {
    fmt.Println("Task 1...")
    hexInput := "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"
    base64Expected := "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"

    base64Real, _ := StrHexToBase64(hexInput)
    if base64Expected != base64Real {
        panic("Set 1 Tasl 1 failed")
    }
    if isVerbose {
        fmt.Println(base64Real)
    }
    fmt.Println("Task 1 Done.")
}


func main() {
    isVerbose := true
    fmt.Println("Set 1...  ")
    task1(isVerbose)
    fmt.Println("Set 1 Done.")
}
