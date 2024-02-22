package util

import (
	"encoding/gob"
	"fmt"
	"log"
	"os"
)

func DecodeColorDatabase(dbPath string) map[string]Rgb {
	//fmt.Printf("debug in decoding")
	file, err := os.Open(dbPath)
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	var database map[string]Rgb //no util. needed as we're in that package?

	decoder := gob.NewDecoder(file)
	if err := decoder.Decode(&database); err != nil {
		log.Fatal(err)
	}

	return database
}

func PrintColorDatabase(dbPath string) {
	fmt.Println(DecodeColorDatabase(dbPath))
}
