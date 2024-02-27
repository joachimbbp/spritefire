package sprite

import (
	"encoding/gob"
	"fmt"
	"log"
	"os"
	"path/filepath"

	"image"
	_ "image/png"

	"github.com/joachimbbp/spritefire/src/util"
)

func Database(input string, output string) {
	fmt.Println("Building database ...")
	fmt.Println("Converting sprites in %s and saving to output %s", input, output)

	spriteColorDatabase := make(map[string]util.Rgb)

	sprites, err := os.ReadDir(input)
	if err != nil {
		log.Fatal(err)
	}
	for _, entry := range sprites {
		if !entry.IsDir() {
			if filepath.Ext(entry.Name()) != ".png" {
				continue
			}
			filePath := filepath.Join(input, entry.Name())
			spriteColorDatabase[entry.Name()] = averageColor(filePath)
		}
	}

	file, err := os.Create(filepath.Join(util.DatabaseFolderPath, "sprite_color_db"))
	if err != nil {
		log.Fatal(err)
	}

	defer file.Close()

	encoder := gob.NewEncoder(file)
	if err := encoder.Encode(spriteColorDatabase); err != nil {
		log.Fatal(err)
	}
}

func averageColor(imagePath string) util.Rgb {
	file, err := os.Open(imagePath)
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	img, _, err := image.Decode(file)
	if err != nil {
		log.Fatal(err)
	}

	rTotal, gTotal, bTotal := 0, 0, 0
	bounds := img.Bounds()
	pixelTotal := bounds.Dx() * bounds.Dy()

	for y := bounds.Min.Y; y < bounds.Max.Y; y++ {
		for x := bounds.Min.X; x < bounds.Max.X; x++ {
			rgbcolor := util.GetRGB(x, y, img)
			rTotal += int(rgbcolor.R)
			gTotal += int(rgbcolor.G)
			bTotal += int(rgbcolor.B)
		}
	}

	var average util.Rgb
	average.R = int(rTotal / pixelTotal)
	average.G = int(gTotal / pixelTotal)
	average.B = int(bTotal / pixelTotal)

	fmt.Printf("average color is: %d\n", average)

	return average
}