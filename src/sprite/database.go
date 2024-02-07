package sprite

import (
	"fmt"
	"log"
	"os"
)

type RGB struct {
	R int
	G int
	B int
} //later maybe go in util, maybe needs to be linear anyways

func Database(input string, output string) {
	fmt.Printf("Sprite Database Function Executing\n Converting Sprites in %s and saving to output %s", input, output)
	sprites, err := os.ReadDir(input)
	if err != nil {
		fmt.Printf("Fatal error %s", err)
		log.Fatal(err) //fatal might be a bit much if you make this into a cmd exe
	}
	for _, entry := range sprites { //study this syntax, you don't quite get it
		if !entry.IsDir() { //checks to make sure it's a file, not a directory
			fmt.Println(entry.Name()) //just prints for now
		}
	}
}

//avereage color, local function
