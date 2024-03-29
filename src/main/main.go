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
		fmt.Println("args:\ndatabase\nprintdb\nresize\nvideo <sprite size index> <image sequence path>\nbatch_res <image sequence path>\nCI_testing\nfull_offline_test")
		fmt.Println("Sprite Sizes by Index:")
		fmt.Println("0: 120\n1: 80\n2: 60\n3: 48\n4: 40\n5: 30\n6: 24\n7: 16\n8: 15\n9: 12")
		//			{120, 		80, 	60, 	48,   40,    30,   24,    16,    15,    12}
		fmt.Println("See readme for more information about how to use this program")
		return
	}
	mode := os.Args[1]
	inputSequence := util.SequencePath

	var spriteSizeIndex int

	if len(os.Args) == 4 && mode == "video" {
		spriteSizeIndex, _ = strconv.Atoi(os.Args[2])
		inputSequence = os.Args[3]
	}
	if len(os.Args) == 3 && mode == "batch_res" {
		inputSequence = os.Args[2]
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
		fmt.Println(inputSequence, spriteSizeIndex)

		util.TimeIt(
			"Generating Video",
			video.Sequence,
			inputSequence,
			util.DatabasePath,
			spriteSizeIndex,
		)
	case "batch_res":
		//batch res is unstable
		fmt.Println("Generating video for multiple resolutions")
		batchResIndices := []int{0, 1, 2, 3, 4, 5, 6, 7, 8, 9}
		util.TimeIt(
			"Generating video for multiple resolutions",
			video.BatchSequence,
			inputSequence,
			util.DatabasePath,
			batchResIndices,
		)
	case "CI_testing":
		fmt.Println("Testing for github actions")
		sprite.Database(util.SpriteInput, util.DatabaseFolderPath, true)
		sprite.Resize(util.SpriteInput, util.SpriteSizes, true)
		/*raylib functions use a window and thus cannot be run with github actions
		Thus this only tests if the database will run.
		For a more robust test, I reccomend using the full_offline_test on your local machine*/

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

		fmt.Println("Generating video for multiple resolutions")
		batchResIndices := []int{0, 1, 2, 3, 4, 5, 6, 7, 8, 9}
		batchResTime, _, _ := util.TimeIt(
			"Generating video for multiple resolutions",
			video.BatchSequence,
			util.SequencePath,
			util.DatabasePath,
			batchResIndices,
		)

		totalTime := dbTime + resizeTime + batchResTime
		fmt.Println("Total Time for full offline test: ", totalTime.Minutes(), "minutes")
	}
}
