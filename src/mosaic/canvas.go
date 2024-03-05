package mosaic

import (
	"bytes"
	"fmt"
	"image"
	"log"
	"math"
	"sort"

	"github.com/joachimbbp/spritefire/src/search"
	"github.com/joachimbbp/spritefire/src/util"
	"gopkg.in/h2non/bimg.v1" //remove later?
)

//var spriteColorDb map[string]util.Rgb

func Canvas(imagePath string, spriteColorDb map[string]util.Rgb, spriteSizeIndex int, tree *search.KDTree) []util.IndexedSprite {
	spriteSize := util.ResizeResolutions[spriteSizeIndex]

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

	fmt.Printf("Creating canvas for %s\nSprite Size: %d\nResolution %d X %d", imagePath, spriteSize, x_tiles, y_tiles)

	for y := bounds.Min.Y; y < bounds.Max.Y; y++ {
		for x := bounds.Min.X; x < bounds.Max.X; x++ {
			rgbaColor := util.GetRGB(x, y, imgData)
			//canvas[index] = util.IndexedSprite{Index: index, Sprite: naiveSpriteMatch(rgbaColor, spriteColorDb)}
			canvas[index] = util.IndexedSprite{Index: index, Sprite: search.KdMatchTileToSprite(rgbaColor.R, rgbaColor.G, rgbaColor.B, tree)}
			//fmt.Printf("matched %d to %s", index, canvas[index].Sprite)
			index++
		}
	}
	sort.Slice(canvas, func(i, j int) bool {
		return canvas[i].Index < canvas[j].Index
	})

	return canvas

}

func naiveSpriteMatch(cell util.Rgb, database map[string]util.Rgb) string {

	closestSprite := "initialized value"

	// Largest possible distance in 8-bit RGB space
	shortestColorLength := math.Sqrt(3 * math.Pow(255, 2))

	for entry, sprite := range database {
		redDistTemp := math.Pow(float64(sprite.R-cell.R), 2)
		greenDistTemp := math.Pow(float64(sprite.G-cell.G), 2)
		blueDistTemp := math.Pow(float64(sprite.B-cell.B), 2)

		distance := math.Sqrt(redDistTemp + greenDistTemp + blueDistTemp)

		if distance < shortestColorLength {
			shortestColorLength = distance
			closestSprite = entry
		}
	}

	return closestSprite
}
