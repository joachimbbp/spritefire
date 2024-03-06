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

//TODO: multithread in goroutines

func Database(spriteFolder string, outputFolder string) {
	util.CreateIfNotExist(outputFolder)

	fmt.Println("Building database ...")
	fmt.Printf("Converting sprites in %s and saving to output %s", spriteFolder, outputFolder)

	//spriteColorDatabase := make(map[string]util.Rgba)
	spriteColorDatabase := make(map[string]util.Rgb)
	sprites, err := os.ReadDir(spriteFolder)
	if err != nil {
		log.Fatal(err)
	}
	for _, entry := range sprites {
		if !entry.IsDir() {
			if filepath.Ext(entry.Name()) != ".png" {
				continue
			}
			filePath := filepath.Join(spriteFolder, entry.Name())
			spriteColorDatabase[entry.Name()] = averageColorRgb(filePath)
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

func averageColorRgb(imagePath string) util.Rgb {
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
			rgbaColor := util.GetRGBA(x, y, img)
			rTotal += int(rgbaColor.R)
			gTotal += int(rgbaColor.G)
			bTotal += int(rgbaColor.B)

		}
	}

	var average util.Rgb
	average.R = int(rTotal / pixelTotal)
	average.G = int(gTotal / pixelTotal)
	average.B = int(bTotal / pixelTotal)

	fmt.Printf("average color for %s is: %d\n", filepath.Base(imagePath), average)

	return average
}

func averageColorRgba(imagePath string) util.Rgba {
	file, err := os.Open(imagePath)
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	img, _, err := image.Decode(file)
	if err != nil {
		log.Fatal(err)
	}

	rTotal, gTotal, bTotal, aTotal := 0, 0, 0, 0
	bounds := img.Bounds()
	pixelTotal := bounds.Dx() * bounds.Dy()

	for y := bounds.Min.Y; y < bounds.Max.Y; y++ {
		for x := bounds.Min.X; x < bounds.Max.X; x++ {
			rgbaColor := util.GetRGBA(x, y, img)
			rTotal += int(rgbaColor.R)
			gTotal += int(rgbaColor.G)
			bTotal += int(rgbaColor.B)
			aTotal += int(rgbaColor.A)
		}
	}

	var average util.Rgba
	average.R = int(rTotal / pixelTotal)
	average.G = int(gTotal / pixelTotal)
	average.B = int(bTotal / pixelTotal)
	average.A = int(aTotal / pixelTotal)

	fmt.Printf("average color for %s is: %d\n", filepath.Base(imagePath), average)

	return average
}
