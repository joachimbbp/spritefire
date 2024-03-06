package mosaic

import (
	"fmt"
	"path/filepath"
	"strconv"

	rl "github.com/gen2brain/raylib-go/raylib"
	"github.com/joachimbbp/spritefire/src/util"
)

func Draw(canvas []util.IndexedSprite, frameName string, spriteSizeIndex int) {
	fmt.Println("Drawing")

	spriteSize := util.ResizeResolutions[spriteSizeIndex]
	util.CreateIfNotExist(util.ImageOutput)
	texX := int32(util.SaveResolutionX)
	texY := int32(util.SaveResolutionY)
	rl.InitWindow(texX, texY, frameName)
	defer rl.CloseWindow()
	targetTexture := rl.LoadRenderTexture(texX, texY)

	fmt.Println("targetTexture intialized")
	rl.BeginTextureMode(targetTexture)
	oX := int32(0)
	oY := texY - int32(spriteSize)

	fmt.Println("image intitialized, for loop begins execution")

	for _, tile := range canvas {

		tilePath := filepath.Join(util.SpriteSizes, strconv.Itoa(spriteSize), tile.Sprite)
		tileTexture := rl.LoadTexture(tilePath)

		sourceRec := rl.NewRectangle(0, 0, float32(tileTexture.Width), -float32(tileTexture.Height))
		destRec := rl.NewRectangle(float32(oX), float32(oY), float32(tileTexture.Width), float32(tileTexture.Height))
		origin := rl.NewVector2(0, 0)

		rl.DrawTexturePro(tileTexture, sourceRec, destRec, origin, 0, rl.White)

		bounds := util.SaveResolutionX - spriteSize
		if oX >= int32(bounds) {
			oX = 0
			oY -= int32(spriteSize)
		} else {
			oX += int32(spriteSize)
		}
	}
	fmt.Println(targetTexture)
	fmt.Println("^ targetTexture")
	rl.EndTextureMode()

	img := rl.LoadImageFromTexture(targetTexture.Texture)

	fmt.Println(img)
	rl.ExportImage(*img, util.ImageOutput+"/"+frameName)
	rl.UnloadImage(img)

}
