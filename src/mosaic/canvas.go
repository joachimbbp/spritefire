package mosaic

import (
	"bytes"
	"fmt"
	"image"
	"log"
	"math"
	"sort"

	"github.com/joachimbbp/spritefire/src/util"
	"gopkg.in/h2non/bimg.v1"
)

func Canvas(imagePath string, spriteColorDbPath string, spriteSize int) []util.IndexedSprite {
	fmt.Printf("Creating canvas for %s\n", imagePath)
	x_tiles := util.SaveResolutionX / spriteSize
	y_tiles := util.SaveResolutionY / spriteSize

	buffer, err := bimg.Read(imagePath)
	if err != nil {
		log.Fatal(err)
	}

	resizedBuffer, err := bimg.NewImage(buffer).Resize(x_tiles, y_tiles)
	if err != nil {
		log.Fatal(err)
	}
	imgData, _, err := image.Decode(bytes.NewReader(resizedBuffer))
	if err != nil {
		log.Fatal(err)
	}
	bounds := imgData.Bounds()
	index := 0
	canvas := make([]util.IndexedSprite, x_tiles*y_tiles)
	fmt.Printf("Matching Sprites\n")
	for y := bounds.Min.Y; y < bounds.Max.Y; y++ {
		for x := bounds.Min.X; x < bounds.Max.X; x++ {
			rgbaColor := util.GetRGB(x, y, imgData) //imgData maybe bad var name
			canvas[index] = util.IndexedSprite{Index: index, Sprite: spriteMatch(rgbaColor, spriteColorDbPath)}
			fmt.Printf("matched %d to %s", index, canvas[index].Sprite)
			index++
		}
	}
	sort.Slice(canvas, func(i, j int) bool { //STUDY THIS
		return canvas[i].Index < canvas[j].Index
	})

	return canvas

}

func spriteMatch(cell util.Rgb, spriteColorDbPath string) string {
	//Copilot refactored from original python code
	//LOTS of TODO here
	//Needs 3D binary search
	//Keeping it this way for now to see how fast Go is even with slow code

	colorDistances := make(map[string]float64)

	database := util.DecodeColorDatabase(spriteColorDbPath)

	for entry, sprite := range database {
		redDistTemp := math.Pow(float64(sprite.R-cell.R), 2)
		greenDistTemp := math.Pow(float64(sprite.G-cell.G), 2)
		blueDistTemp := math.Pow(float64(sprite.B-cell.B), 2)
		colorDistances[entry] = math.Sqrt(redDistTemp + greenDistTemp + blueDistTemp)
	}
	closestSprite := "initialized value"
	shortestColorLength := 17367040.0 // Hard coded for 8-bit
	for sprite, distance := range colorDistances {
		if distance < shortestColorLength {
			shortestColorLength = distance
			closestSprite = sprite // match to proper resolution
		}
	}

	return closestSprite
}
