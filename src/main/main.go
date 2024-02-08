package main

import (
	"fmt"
	"os"
	"time"

	"github.com/joachimbbp/spritefire/src/sprite"
	"github.com/joachimbbp/spritefire/src/util"
)

func main() {
	if len(os.Args) < 2 {
		fmt.Println("usage: main.go <arg>")
		fmt.Println("Args: database, printdb, resize")
		return
	}

	mode := os.Args[1] //user setable based on what you would like to execute
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
		sprite.Resize(util.SpriteInput, util.SpriteOutput)
	}
}
