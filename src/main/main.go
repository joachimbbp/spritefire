package main

import (
	"fmt"
	"os"

	"github.com/joachimbbp/spritefire/src/sprite"
	"github.com/joachimbbp/spritefire/src/util"
	"github.com/joachimbbp/spritefire/src/video"
)

func main() {
	if len(os.Args) < 2 {
		fmt.Println("usage: main.go <arg>")
		fmt.Println("args: database, printdb, resize")
		return
	}

	mode := os.Args[1]

	switch mode {
	case "database":
		util.TimeIt(
			"database creation",
			sprite.Database,
			util.SpriteInput,
			util.DatabaseFolderPath,
		)

	case "printdb":
		fmt.Printf("printing db\n")
		util.PrintColorDatabase("/Users/joachimpfefferkorn/Documents/GitHub/spritefire/ignore/database/sprite_color_db")

	case "resize":
		util.TimeIt(
			"resizing",
			sprite.Resize,
			util.SpriteInput,
			util.SpriteSizes,
		)

	case "video":
		util.TimeIt(
			"generate video",
			video.Sequence,
			util.SequencePath,
			util.DatabasePath,
			5,
		)
	}
}
