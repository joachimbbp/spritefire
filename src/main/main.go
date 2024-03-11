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
		fmt.Println("Creating database")
		util.TimeIt(
			"database creation",
			sprite.Database,
			util.SpriteInput,
			util.DatabaseFolderPath,
		)

	case "printdb":
		util.PrintColorDatabase("/Users/joachimpfefferkorn/Documents/GitHub/spritefire/ignore/database/sprite_color_db")

	case "resize":
		spriteResIndices := []int{5} //hard coded at just 5 for CI for no
		fmt.Println("Resizing Sprites to resolutions:")
		for _, index := range spriteResIndices {
			fmt.Println(util.SpriteSizes[index])
		}
		util.TimeIt(
			"resizing",
			sprite.Resize,
			util.SpriteInput,
			util.SpriteSizes,
			spriteResIndices,
		)

	case "video":
		fmt.Println("Generating Video")
		util.TimeIt(
			"Generating Video",
			video.Sequence,
			util.SequencePath,
			util.DatabasePath,
			5, //set this to choose the desired resolution. Hard coded at 5 for CI for now
		)
	case "batchRes":
		fmt.Println("Generating video for multiple resolutions")
		batchResIndices := []int{0, 1, 2, 3, 4, 5, 6, 7, 8, 9}
		util.TimeIt(
			"Generating video for multiple resolutions",
			video.BatchSequence,
			util.SequencePath,
			util.DatabasePath,
			batchResIndices,
		)

	}
}
