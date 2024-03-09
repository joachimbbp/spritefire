package mosaic

import (
	"fmt"
	"path/filepath"
	"strconv"

	rl "github.com/gen2brain/raylib-go/raylib"
	"github.com/joachimbbp/spritefire/src/util"
)

func Draw(canvas []util.IndexedSprite, prefix string, frameName string, spriteSizeIndex int) {
	spriteSize := util.ResizeResolutions[spriteSizeIndex]

	fmt.Println("\nDrawing and saving image to disk")
	fmt.Println("frame:\n", frameName)
	fmt.Println("sprite size:\n", spriteSize)

	util.CreateIfNotExist(util.ImageOutput)
	texX := int32(util.SaveResolutionX)
	texY := int32(util.SaveResolutionY)
	rl.InitWindow(texX, texY, frameName)
	defer rl.CloseWindow()
	targetTexture := rl.LoadRenderTexture(texX, texY)

	rl.BeginTextureMode(targetTexture)
	oX := int32(0)
	oY := texY - int32(spriteSize)

	for _, tile := range canvas {
		if tile.Sprite != "blanktile" {
			drawSprite(tile, spriteSize, oX, oY)
		}

		bounds := util.SaveResolutionX - spriteSize
		if oX >= int32(bounds) {
			oX = 0
			oY -= int32(spriteSize)
		} else {
			oX += int32(spriteSize)
		}
	}
	rl.EndTextureMode()

	img := rl.LoadImageFromTexture(targetTexture.Texture)

	rl.ExportImage(*img, util.ImageOutput+"/"+prefix+frameName)
	rl.UnloadImage(img)
	rl.UnloadRenderTexture(targetTexture)
}

func drawSprite(tile util.IndexedSprite, spriteSize int, oX int32, oY int32) {
	tilePath := filepath.Join(util.SpriteSizes, strconv.Itoa(spriteSize), tile.Sprite)
	tileTexture := rl.LoadTexture(tilePath)

	sourceRec := rl.NewRectangle(0, 0, float32(tileTexture.Width), -float32(tileTexture.Height))
	destRec := rl.NewRectangle(float32(oX), float32(oY), float32(tileTexture.Width), float32(tileTexture.Height))
	origin := rl.NewVector2(0, 0)

	rl.DrawTexturePro(tileTexture, sourceRec, destRec, origin, 0, rl.White)
}
