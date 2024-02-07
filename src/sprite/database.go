package sprite

import (
	"fmt"
	"log"
	"os"
	"path/filepath"

	"image"
	"image/color"
	_ "image/png"

	"github.com/joachimbbp/spritefire/src/util"
)

func Database(input string, output string) {
	fmt.Printf("Sprite Database Function Executing\n Converting Sprites in %s and saving to output %s", input, output)
	sprites, err := os.ReadDir(input)
	if err != nil {
		log.Fatal(err) //fatal might be a bit much if you make this into a cmd exe
	}
	for _, entry := range sprites { //study this syntax
		if !entry.IsDir() {
			//fmt.Println(entry.Name()) //just prints for now
			filePath := filepath.Join(input, entry.Name())
			//TODO maybe need a check that it's a PNG file?
			averageColor(filePath)
		}
	}
}

//avereage color, local function

func averageColor(imagePath string) {
	//fmt.Printf("avereageColor called for %s\n", imagePath)

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
	pixelTotal := bounds.Dx() * bounds.Dy() //total amount of pixels

	for y := bounds.Min.Y; y < bounds.Max.Y; y++ {
		for x := bounds.Min.X; x < bounds.Max.X; x++ {
			rgbColor := color.RGBAModel.Convert(img.At(x, y)).(color.RGBA) //study this syntax
			rTotal += int(rgbColor.R)
			gTotal += int(rgbColor.G)
			bTotal += int(rgbColor.B)
		}
	}
	var average util.Rgb
	average.R = int(rTotal / pixelTotal)
	average.G = int(gTotal / pixelTotal)
	average.B = int(bTotal / pixelTotal)
	fmt.Printf("average color is: %d\n", average)

}
