package main

import (
	"fmt"
	"os"
	"time"

	"github.com/joachimbbp/spritefire/src/mosaic"
	"github.com/joachimbbp/spritefire/src/sprite"
	"github.com/joachimbbp/spritefire/src/util"
	"github.com/joachimbbp/spritefire/src/video"
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
		sprite.Database(util.SpriteInput, util.SpriteSizes)
		dbEnd := time.Now()
		fmt.Printf("database created in %s", dbEnd.Sub(dbStart))
	case "printdb":
		fmt.Printf("printing db\n")
		util.PrintColorDatabase("/Users/joachimpfefferkorn/Documents/GitHub/spritefire/ignore/database/sprite_color_db")
	case "resize":
		rsStart := time.Now()
		sprite.Resize(util.SpriteInput, util.SpriteSizes)
		rsEnd := time.Now()
		fmt.Printf("resizing done in %s", rsEnd.Sub(rsStart))
	case "draw":
		dStart := time.Now()
		sRes := 5
		//gonna have to open the canvas map here
		mosaic.Draw(mosaic.Canvas(util.InputStill, util.DatabasePath, util.ResizeResolutions[sRes]), "test_frame", util.ResizeResolutions[sRes])
		dEnd := time.Now()
		fmt.Printf("drawn in %s", dEnd.Sub(dStart))

	case "video":
		vStart := time.Now()
		video.Sequence(util.SequencePath, util.DatabaseFolderPath, 1)
		vEnd := time.Now()
		fmt.Printf("video done in %s", vEnd.Sub(vStart))
	}
}