package mosaic

import (
	"bytes"
	"fmt"
	"image"
	"log"
	"sort"

	"github.com/joachimbbp/spritefire/src/search"
	"github.com/joachimbbp/spritefire/src/util"
	"gopkg.in/h2non/bimg.v1"
)

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
			rgbaColor := util.GetRGBA(x, y, imgData)
			if rgbaColor.A != 0 { // Skip fully transparent pixels
				canvas[index] = util.IndexedSprite{
					Index:  index,
					Sprite: search.KdMatchTileToSprite(rgbaColor.R, rgbaColor.G, rgbaColor.B, tree),
				}
			} else {
				canvas[index] = util.IndexedSprite{
					Index:  index,
					Sprite: "blanktile",
				}
			}
			index++
		}
	}

	sort.Slice(canvas, func(i, j int) bool {
		return canvas[i].Index < canvas[j].Index
	})

	return canvas

}
