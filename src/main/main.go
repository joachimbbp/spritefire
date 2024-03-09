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
		fmt.Println("args: database, printdb, resize, video")
		fmt.Println("See readme for more information about how to use this program")
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
			"generating video",
			video.Sequence,
			util.SequencePath,
			util.DatabasePath,
			5, //set this to choose the desired resolution. See readme for more
			false,
		)
	case "batchRes":
		batchResIndices := []int{0, 4, 9}
		util.TimeIt(
			"generating video for multiple resolutions",
			video.BatchSequence,
			util.SequencePath,
			util.DatabasePath,
			batchResIndices,
		)

	}
}
