package sprite

import (
	"fmt"
	"log"
	"os"
	"path/filepath"

	"github.com/joachimbbp/spritefire/src/util"
	"gopkg.in/h2non/bimg.v1"
)

//TODO run in parallel with goroutines or (better yet) raylib

func Resize(spriteFolder string, resizedFolder string) {
	util.CreateIfNotExist(resizedFolder)
	fmt.Println("Resizing sprites ...")

	for i, resolution := range util.ResizeResolutions {
		sprites, readDirErr := os.ReadDir(spriteFolder)
		if readDirErr != nil {
			log.Fatal(readDirErr)
		}

		fmt.Printf("resizing for resolution %d: %d\n", i, resolution)
		resSubfolder := filepath.Join(resizedFolder, fmt.Sprint(resolution))

		fmt.Printf("folder created: %s\n", resSubfolder)
		mkdirErr := os.Mkdir(resSubfolder, 0755)
		if mkdirErr != nil {
			log.Fatal(mkdirErr)
		}

		for _, sprite := range sprites {
			fmt.Printf("resizing %s to resolution %d\n", sprite, resolution)

			spritePath := filepath.Join(spriteFolder, sprite.Name())

			buffer, err := bimg.Read(spritePath)
			if err != nil {
				log.Fatal(err)
			}

			thumbnailImage, err := bimg.NewImage(buffer).Thumbnail(resolution)
			if err != nil {
				log.Fatal(err)
			}

			fmt.Printf("Opened %s\n", sprite)
			fmt.Printf("Image created saved at size %d\n", resolution)

			outPath := resizedFolder + "/" + fmt.Sprint(resolution) + "/" + sprite.Name()
			err = bimg.Write(outPath, thumbnailImage)
			if err != nil {
				log.Fatal(err)
			}
		}

	}
}
