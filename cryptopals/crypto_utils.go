package main

import (
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
