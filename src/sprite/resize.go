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

func Resize(spriteFolder string, resizedFolder string, ci bool) {
	util.CreateIfNotExist(resizedFolder)

	for _, resolution := range util.ResizeResolutions {
		sprites, readDirErr := os.ReadDir(spriteFolder)
		if readDirErr != nil {
			log.Fatal(readDirErr)
		}

		resSubfolder := filepath.Join(resizedFolder, fmt.Sprint(resolution))

		mkdirErr := os.Mkdir(resSubfolder, 0755)
		if mkdirErr != nil {
			log.Fatal(mkdirErr)
		}

		for _, sprite := range sprites {
			if !ci {
				fmt.Println("\nCreating Resized Sprite")
				fmt.Println("Resolution:\n", resolution)
				fmt.Println("Sprite:\n", sprite)
			}

			spritePath := filepath.Join(spriteFolder, sprite.Name())

			buffer, err := bimg.Read(spritePath)
			if err != nil {
				log.Fatal(err)
			}

			thumbnailImage, err := bimg.NewImage(buffer).Thumbnail(resolution)
			if err != nil {
				log.Fatal(err)
			}

			outPath := resizedFolder + "/" + fmt.Sprint(resolution) + "/" + sprite.Name()
			err = bimg.Write(outPath, thumbnailImage)
			if err != nil {
				log.Fatal(err)
			}
		}

	}
}
