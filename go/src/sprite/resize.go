package sprite

import (
	"fmt"
	"log"
	"os"
	"path/filepath"

	"github.com/joachimbbp/spritefire/src/util"
	"gopkg.in/h2non/bimg.v1"
)

func Resize(input string, output string) {
	fmt.Println("Resizing sprites")
	for i, resolution := range util.ResizeResolutions {
		sprites, readDirErr := os.ReadDir(input)
		if readDirErr != nil {
			log.Fatal(readDirErr)
		}

		fmt.Printf("resizing for resolution %d: %d\n", i, resolution)
		resSubfolder := filepath.Join(output, fmt.Sprint(resolution))
		fmt.Printf("folder created: %s\n", resSubfolder) //check formatting
		mkdirErr := os.Mkdir(resSubfolder, 0755)
		if mkdirErr != nil {
			log.Fatal(mkdirErr)
		}
		//perhaps un-nesting this will make it faster...
		for _, sprite := range sprites {
			fmt.Printf("resizing %s to resolution %d\n", sprite, resolution)
			spritePath := filepath.Join(input, sprite.Name())
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
			err = bimg.Write(output+"/"+fmt.Sprint(resolution)+"/"+sprite.Name(), thumbnailImage)
			if err != nil {
				log.Fatal(err)
			}
		}

	}
}
