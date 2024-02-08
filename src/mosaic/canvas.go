package mosaic

import (
	"fmt"
	"log"

	"github.com/joachimbbp/spritefire/src/util"
	"gopkg.in/h2non/bimg.v1"
)

func Canvas(imagePath string, databse string, spriteSize int) {
	fmt.Printf("Creating canvas for %s", imagePath)
	x_tiles := util.SaveResolutionX / spriteSize
	y_tiles := util.SaveResolutionY / spriteSize

	buffer, err := bimg.Read(imagePath)
	if err != nil {
		log.Fatal(err)
	}

	colorDataImg, err := bimg.NewImage(buffer).Resize(x_tiles, y_tiles)
	if err != nil {
		log.Fatal(err)
	}
	err = bimg.Write(util.ScratchOutput+"/"+"colorDataImg.png", colorDataImg)
	if err != nil {
		log.Fatal(err)
	}

}
