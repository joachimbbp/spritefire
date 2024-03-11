package main

import (
	"fmt"
	"os"
	"strconv"

	"github.com/joachimbbp/spritefire/src/sprite"
	"github.com/joachimbbp/spritefire/src/util"
	"github.com/joachimbbp/spritefire/src/video"
)

func main() {
	if len(os.Args) < 2 {
		fmt.Println("Usage: main.go <arg> <sprite size index>")
		fmt.Println("args: database, printdb, resize, video <sprite size index>, batchRes, CI_testing, full_offline_test")
		fmt.Println("Sprite Sizes by Index:")
		fmt.Println("0: 120\n1: 80\n2: 60\n3: 48\n4: 40\n5: 30\n6: 24\n7: 16\n8: 15\n9: 12")
		//			{120, 		80, 	60, 	48,   40,    30,   24,    16,    15,    12}
		fmt.Println("See readme for more information about how to use this program")
		return
	}

	mode := os.Args[1]
	var spriteSizeIndex int
	if len(os.Args) == 3 {
		spriteSizeIndex, _ = strconv.Atoi(os.Args[2])
	}

	switch mode {
	case "database":
		fmt.Println("Creating database")
		util.TimeIt(
			"database creation",
			sprite.Database,
			util.SpriteInput,
			util.DatabaseFolderPath,
			false,
		)

	case "resize":
		fmt.Println("Resizing Sprites to resolutions:")

		util.TimeIt(
			"resizing",
			sprite.Resize,
			util.SpriteInput,
			util.SpriteSizes,
			false,
		)

	case "video":
		fmt.Println("Generating Video")
		util.TimeIt(
			"Generating Video",
			video.Sequence,
			util.SequencePath,
			util.DatabasePath,
			spriteSizeIndex,
		)
	case "batchRes":
		//batch res is unstable
		fmt.Println("Generating video for multiple resolutions")
		batchResIndices := []int{0, 1, 2, 3, 4, 5, 6, 7, 8, 9}
		util.TimeIt(
			"Generating video for multiple resolutions",
			video.BatchSequence,
			util.SequencePath,
			util.DatabasePath,
			batchResIndices,
		)
	case "CI_testing":
		fmt.Println("Testing for github actions")
		sprite.Database(util.SpriteInput, util.DatabaseFolderPath, true)
		sprite.Resize(util.SpriteInput, util.SpriteSizes, true)
		/*raylib functions use a window and thus cannot be run with github actions
		Thus this only tests if the database will run*/

	case "full_offline_test":
		fmt.Println("Full Offline Test")
		//TODO: is there a way to dry this?

		dbTime, _, _ := util.TimeIt(
			"database creation",
			sprite.Database,
			util.SpriteInput,
			util.DatabaseFolderPath,
			false,
		)
		resizeTime, _, _ := util.TimeIt(
			"resizing",
			sprite.Resize,
			util.SpriteInput,
			util.SpriteSizes,
			false,
		)

		videoTime, _, _ := util.TimeIt(
			"Generating Video",
			video.Sequence,
			util.SequencePath,
			util.DatabasePath,
			5, //Hard coded at 5 for CI for now. Eventually this should be a batch video (see dev notes)
		)
		totalTime := dbTime + resizeTime + videoTime
		fmt.Println("Total Time for full offline test: ", totalTime.Minutes(), "minutes")
	}
}
