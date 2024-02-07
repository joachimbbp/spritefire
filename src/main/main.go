package main

import (
	"fmt"

	"github.com/joachimbbp/spritefire/src/sprite"
)

var spriteInput string = "/Users/joachimpfefferkorn/Documents/GitHub/sprite_mosaic/test_files/sprites_512" //why can't I use that nice := thing here???
var spriteOutput string = "spam"

func main() {
	fmt.Print("Main Function executing\n")

	mode := "database" //user setable based on what you would like to execute
	switch mode {
	case "database":
		sprite.Database(spriteInput, spriteOutput)

	case "resize":
		sprite.Resize()
	}
}
