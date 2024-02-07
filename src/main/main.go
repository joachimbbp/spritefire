package main

import (
	"fmt"
	"time"

	"github.com/joachimbbp/spritefire/src/sprite"
	"github.com/joachimbbp/spritefire/src/util"
)

//maybe add a cl arg that adjusts this switch?

func main() {
	fmt.Print("Main Function executing\n")

	mode := "printdb" //user setable based on what you would like to execute
	switch mode {
	case "database":
		dbStart := time.Now()
		sprite.Database(util.SpriteInput, util.SpriteOutput)
		dbEnd := time.Now()
		fmt.Printf("database created in %s seconds", dbEnd.Sub(dbStart))

	case "printdb":
		fmt.Printf("printing db\n")
		util.DecodeColorDatabase("/Users/joachimpfefferkorn/Documents/GitHub/spritefire/ignore/database/sprite_color_db")
	case "resize":
		sprite.Resize()
	}
}
