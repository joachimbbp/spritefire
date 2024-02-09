package mosaic

//Full up GPT nonsense from original Python code
//Using for testing but PLEASE write a better one later, for the sake of your soul

import (
	"image"
	"image/draw"
	"image/png"
	"log"
	"os"
	"path/filepath"
	"strconv"

	"github.com/joachimbbp/spritefire/src/util"
)

func Draw(canvas []util.IndexedSprite, frameName string, spriteSize int) {
	tilePosX, tilePosY := 0, 0
	mosaic := image.NewRGBA(image.Rect(0, 0, util.SaveResolutionX, util.SaveResolutionY))

	for _, tile := range canvas {
		currentTileFile, err := os.Open(filepath.Join(util.SpriteSizes, strconv.Itoa(spriteSize), tile.Sprite))
		if err != nil {
			log.Fatal(err)
		}
		defer currentTileFile.Close()

		currentTile, _, err := image.Decode(currentTileFile)
		if err != nil {
			log.Fatal(err)
		}

		r := image.Rect(tilePosX, tilePosY, tilePosX+spriteSize, tilePosY+spriteSize)
		draw.Draw(mosaic, r, currentTile, image.Point{}, draw.Src)

		if tilePosX >= util.SaveResolutionX-spriteSize {
			tilePosX = 0
			tilePosY += spriteSize
		} else {
			tilePosX += spriteSize
		}
	}

	outputFile, err := os.Create(filepath.Join(util.ScratchOutput, frameName+".png"))
	if err != nil {
		log.Fatal(err)
	}
	defer outputFile.Close()

	err = png.Encode(outputFile, mosaic)
	if err != nil {
		log.Fatal(err)
	}
}
