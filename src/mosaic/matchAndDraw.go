package mosaic

import (
	"fmt"
	"image/color"
	"math"
	"path/filepath"
	"sync"

	rl "github.com/gen2brain/raylib-go/raylib"
	"github.com/joachimbbp/spritefire/src/util"
)

func MatchAndDraw(sourceImagePath string, spriteColorDbPath string, spriteSize int) {

	util.CreateIfNotExist(util.ImageOutput) //even though presently you can't save there with TakeScreenshot
	frameName := filepath.Base(sourceImagePath)

	db := util.DecodeColorDatabase(spriteColorDbPath)
	tree := buildSearchTree(db)

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

	var wg sync.WaitGroup

	//for every pixel
	for y := int32(0); y < sourceImage.Height; y++ {
		for x := int32(0); x < sourceImage.Width; x++ {
			wg.Add(1)
			go func(x, y int32) {
				defer wg.Done()
				drawTile(&sourceColorData, sourceImage, tree,
					x, y, spriteSize)
			}(x, y) // Pass x and y as arguments to the function
		}
	}
	wg.Wait()
	rl.TakeScreenshot(frameName)
}

func drawTile(sourceColorData *[]color.RGBA, sourceImage *rl.Image, tree *KDTree, x int32, y int32, spriteSize int) {

	oX, oY := valToOffset(x, sourceImage)

	// find the closest sprite
	color := (*sourceColorData)[y*sourceImage.Width+x]
	r := int(color.R)
	g := int(color.G)
	b := int(color.B)
	// tile := naiveMatchTileToSprite(r, g, b, db)
	tile := kdMatchTileToSprite(r, g, b, tree)
	tileTexture := rl.LoadTexture(util.SpriteSizes + "/" + fmt.Sprint(spriteSize) + "/" + tile)
	defer rl.UnloadTexture(tileTexture)
	// and draw it to the screen with raylib
	rl.DrawTexture(tileTexture, oX, oY, rl.White)

}

func valToOffset(x int32, sourceImage *rl.Image) (int32, int32) {
	oX := x / sourceImage.Width //oX
	oY := x % sourceImage.Width //oY
	return oX, oY
}

func naiveMatchTileToSprite(r int, g int, b int, spriteColorDb map[string]util.Rgb) string {
	closestSprite := "initialized value naive"
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

func kdMatchTileToSprite(r int, g int, b int, searchTree *KDTree) string {
	color := util.Rgb{R: r, G: g, B: b}

	nearestNode := searchTree.FindNearestNeighbor(color)
	return nearestNode.spriteName

}
