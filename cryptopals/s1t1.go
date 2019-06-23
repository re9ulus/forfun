package main

import (
    "fmt"
    "encoding/hex"
    "encoding/base64"
)


func BinHexToBase64(binHex []byte) []byte {
    base64dst := make([]byte, base64.StdEncoding.EncodedLen(len(binHex)))
    base64.StdEncoding.Encode(base64dst, binHex)
    return base64dst
}


func StrHexToBinHex(strHex string) ([]byte, error) {
    src := []byte(strHex)
    dst := make([]byte, hex.DecodedLen(len(src)))
    _, err := hex.Decode(dst, src)
    if err != nil {
        return nil, err
    }
    return dst, err
}


func StrHexToBase64(strHex string) (string, error) {
    binHex, err := StrHexToBinHex(strHex)
    if err != nil {
        return "", err
    }
    base64dst := BinHexToBase64(binHex)
    return string(base64dst), nil
}


func main() {
    hex_input := "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"
    // base64Expected := "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"

    base64Real, _ := StrHexToBase64(hex_input)
    fmt.Println(base64Real)
}
