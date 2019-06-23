package main

import (
    "encoding/hex"
    "encoding/base64"
)


func BinToBase64(binHex []byte) []byte {
    base64dst := make([]byte, base64.StdEncoding.EncodedLen(len(binHex)))
    base64.StdEncoding.Encode(base64dst, binHex)
    return base64dst
}


func HexToBin(strHex string) []byte {
    src := []byte(strHex)
    dst := make([]byte, hex.DecodedLen(len(src)))
    _, err := hex.Decode(dst, src)
    if err != nil {
        panic("HexToBin failed")
    }
    return dst
}


func HexToBinBase64(strHex string) []byte {
    return BinToBase64(HexToBin(strHex))
}


func HexToBase64(strHex string) string {
    return string(HexToBinBase64(strHex))
}


func XorBinary(a, b []byte) []byte {
    if len(a) != len(b) {
        panic("Binary arrays must have same length")
    }
    res := make([]byte, len(a))
    for idx, _ := range a {
        res[idx] = a[idx] ^ b[idx]
    }
    return res

}


func XorStrings(a, b string) string {
    if len(a) != len(b) {
        panic("String length must be equal")
    }
    binResBase64 := XorBinary(HexToBin(a), HexToBin(b))
    return hex.EncodeToString(binResBase64)
}
