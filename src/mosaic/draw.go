package mosaic

import (
	"path/filepath"
	"strconv"

	rl "github.com/gen2brain/raylib-go/raylib"
	"github.com/joachimbbp/spritefire/src/util"
)

func Draw(canvas []util.IndexedSprite, frameName string, spriteSize int) {
	util.CreateIfNotExist(util.ImageOutput)

	oX := int32(0)
	oY := int32(0)

	rl.InitWindow(util.SaveResolutionX, util.SaveResolutionY, frameName)
	defer rl.CloseWindow()

	for _, tile := range canvas {

		tilePath := filepath.Join(util.SpriteSizes, strconv.Itoa(spriteSize), tile.Sprite)
		tileTexture := rl.LoadTexture(tilePath)

		rl.BeginDrawing()
		rl.DrawTexture(tileTexture, oX, oY, rl.White)

		bounds := util.SaveResolutionX - spriteSize
		if oX >= int32(bounds) {
			oX = 0
			oY += int32(spriteSize)
		} else {
			oX += int32(spriteSize)
		}
	}

	rl.TakeScreenshot(frameName) //no matter what I do this saves to the main dir ugh
	//might have to change the source code to fix this ngl
}
