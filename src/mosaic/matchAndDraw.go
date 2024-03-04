package mosaic

import (
	"fmt"
	"math"
	"path/filepath"

	rl "github.com/gen2brain/raylib-go/raylib"
	"github.com/joachimbbp/spritefire/src/util"
)

func MatchAndDraw(sourceImagePath string, spriteColorDbPath string, spriteSize int) {

	util.CreateIfNotExist(util.ImageOutput) //even though presently you can't save there with TakeScreenshot
	frameName := filepath.Base(sourceImagePath)

	db := util.DecodeColorDatabase(spriteColorDbPath)

	//load in the image and resize it to number of tiles
	xTiles := int32(util.SaveResolutionX / spriteSize)
	yTiles := int32(util.SaveResolutionY / spriteSize)
	sourceImage := rl.LoadImage(sourceImagePath)
	defer rl.UnloadImage(sourceImage)
	rl.ImageResize(sourceImage, xTiles, yTiles)
	sourceColorData := rl.LoadImageColors(sourceImage)

	rl.InitWindow(util.SaveResolutionX, util.SaveResolutionY, frameName)
	defer rl.CloseWindow()
	rl.BeginDrawing()
	oX := int32(0)
	oY := int32(0)

	bounds := util.SaveResolutionX - spriteSize

	//for every pixel
	for y := int32(0); y < sourceImage.Height; y++ {
		for x := int32(0); x < sourceImage.Width; x++ {
			//find the closest sprite
			color := sourceColorData[y*sourceImage.Width+x] //how does this work??? it's from GPT
			r := int(color.R)
			g := int(color.G)
			b := int(color.B)
			tile := matchTileToSprite(r, g, b, db)
			tileTexture := rl.LoadTexture(util.SpriteSizes + "/" + fmt.Sprint(spriteSize) + "/" + tile)
			//and draw it to the screen with raylib
			rl.DrawTexture(tileTexture, oX, oY, rl.White)

			if oX >= int32(bounds) {
				oX = 0
				oY += int32(spriteSize)
			} else {
				oX += int32(spriteSize)
			}

		}
	}
	rl.TakeScreenshot(frameName)
}

func matchTileToSprite(r int, g int, b int, spriteColorDb map[string]util.Rgb) string {
	closestSprite := "initialized value"
	shortestColorLength := math.Sqrt(3 * math.Pow(255, 2))
	for entry, sprite := range spriteColorDb {
		redDistTemp := math.Pow(float64(sprite.R-r), 2)
		greenDistTemp := math.Pow(float64(sprite.G-g), 2)
		blueDistTemp := math.Pow(float64(sprite.B-b), 2)
		distance := math.Sqrt(redDistTemp + greenDistTemp + blueDistTemp)

		if distance < shortestColorLength {
			shortestColorLength = distance
			closestSprite = entry
		}

	}
	return closestSprite
}

func buildSearchTree(spriteColorDB map[string]util.Rgb) *KDTree {
	//grow kd tree for sprites
	//Initial start hard-coded with joker emoji (feels like kinda average values) emoji_u1f0cf.png

	tree := KDTree{}
	for spriteName, color := range spriteColorDB {
		tree.Insert(color, spriteName)
	}
	return &tree
}
