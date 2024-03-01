package util

import (
	"fmt"
	"os"
)

const AssetBasePath = "../../assets"
const OutputBasePath = "../../output"

const DatabaseFolderPath = OutputBasePath + "/database"
const DatabasePath = OutputBasePath + "/database/sprite_color_db"

const SpriteInput = AssetBasePath + "/sprites_512"
const SpriteSizes = AssetBasePath + "/sprite_sizes"
const ScratchOutput = AssetBasePath + "/scratch"

// const CanvasOutput = BasePath + "/canvas"

const InputStill = AssetBasePath + "/test_images/garden_1280x720.png"
const SequencePath = AssetBasePath + "/test_images/GodzillaVsHedorah"

func CreateIfNotExist(dir string) {
	if _, err := os.Stat(dir); os.IsNotExist(err) {
		err := os.MkdirAll(dir, 0755)
		if err != nil {
			fmt.Println("Error creating directory", dir)
			fmt.Println(err)
		} else {
			fmt.Println("directory created: ", dir)
		}
	} else {
		fmt.Println("directory exists", dir)
	}
}
