package sprite

import (
	"fmt"
	"log"
	"os"

	"github.com/joachimbbp/spritefire/src/util"
)

func Resize(input string, output string) {
	fmt.Println("Resizing sprites")
	for i, resolution := range util.ResizeResolutions {
		sprites, readDirErr := os.ReadDir(input)
		if readDirErr != nil {
			log.Fatal(readDirErr)
		}

		fmt.Printf("resizing for resolution %d: %d\n", i, resolution)
		fmt.Printf("folder named: %s\n", fmt.Sprint(resolution)) //check formatting
		mkdirErr := os.Mkdir(fmt.Sprint(resolution), 0755)
		if mkdirErr != nil {
			log.Fatal(mkdirErr)
		}

		for _, sprite := range sprites {
			fmt.Printf("resizing %s to resolution %d", sprite, resolution)
		}

	}
}
