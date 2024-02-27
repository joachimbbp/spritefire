package main

import (
	"fmt"
	"os"

	"github.com/joachimbbp/spritefire/src/mosaic"
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
			util.SpriteSizes,
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

	case "draw":
		//gonna have to open the canvas map here
		sRes := 5
		canvas := mosaic.Canvas(util.InputStill, util.DatabasePath, util.ResizeResolutions[sRes])
		util.TimeIt(
			"draw canvas",
			mosaic.Draw,
			canvas,
			"test_frame",
			util.ResizeResolutions[sRes],
		)

	case "video":
		util.TimeIt(
			"generate video",
			video.Sequence,
			util.SequencePath,
			util.DatabaseFolderPath,
			1,
		)
	}
}
