package util

import (
	"encoding/gob"
	"fmt"
	"log"
	"os"
)

func DecodeColorDatabaseRgba(dbPath string) map[string]Rgba {
	file, err := os.Open(dbPath)
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	var database map[string]Rgba

	decoder := gob.NewDecoder(file)
	if err := decoder.Decode(&database); err != nil {
		log.Fatal(err)
	}

	return database
}

func DecodeColorDatabaseRgb(dbPath string) map[string]Rgb {
	file, err := os.Open(dbPath)
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	var database map[string]Rgb

	decoder := gob.NewDecoder(file)
	if err := decoder.Decode(&database); err != nil {
		log.Fatal(err)
	}

	return database
}

func PrintColorDatabaseRgba(dbPath string) {
	fmt.Println(DecodeColorDatabaseRgba(dbPath))
}
func PrintColorDatabaseRgb(dbPath string) {
	fmt.Println(DecodeColorDatabaseRgb(dbPath))
}
