package util

import (
	"fmt"
	"os"
)

const AssetBasePath = "../../assets"

const TestFootageBasePath = "../../assets/test_footage"
const OutputBasePath = "../../output"

const DatabaseFolderPath = OutputBasePath + "/sprite_mgmt"
const DatabasePath = DatabaseFolderPath + "/sprite_color_db"

const SpriteInput = AssetBasePath + "/sprites_512"
const SpriteSizes = OutputBasePath + "/sprite_mgmt"
const ImageOutput = OutputBasePath + "/image_output"

const SequencePath = TestFootageBasePath + "/scuba" //change this to your own 1280x720 .png sequence

func CreateIfNotExist(dir string) {
	if _, err := os.Stat(dir); os.IsNotExist(err) {
		err := os.MkdirAll(dir, 0755)
		if err != nil {
			fmt.Println("Error creating directory:\n", dir)
			fmt.Println(err)
		} else {
			fmt.Println("directory created:\n", dir)
		}
	} else {
		fmt.Println("directory exists:\n", dir)
	}
}
