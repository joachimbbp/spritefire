package main

import (
	"fmt"
	"os"
	"time"

	"github.com/joachimbbp/spritefire/src/mosaic"
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
		fmt.Printf("database created in %s", dbEnd.Sub(dbStart))
	case "printdb":
		fmt.Printf("printing db\n")
		util.DecodeColorDatabase("/Users/joachimpfefferkorn/Documents/GitHub/spritefire/ignore/database/sprite_color_db")
	case "resize":
		rsStart := time.Now()
		sprite.Resize(util.SpriteInput, util.SpriteOutput)
		rsEnd := time.Now()
		fmt.Printf("resizing done in %s", rsEnd.Sub(rsStart))
	case "canvas":
		cStart := time.Now()
		mosaic.Canvas(util.InputStill, util.DatabasePath, util.ResizeResolutions[1])
		cEnd := time.Now()
		fmt.Printf("resizing done in %s", cEnd.Sub(cStart))
	}
}
