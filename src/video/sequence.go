package video

import (
	"log"
	"os"
	"path/filepath"

	"github.com/joachimbbp/spritefire/src/mosaic"
	"github.com/joachimbbp/spritefire/src/util"
)

func Sequence(sequencePath string, spriteColorDbPath string, spriteSizeIndex int) {
	frames, err := os.ReadDir(sequencePath)
	if err != nil {
		log.Fatal(err)
	}

	for _, frame := range frames {
		if filepath.Ext(frame.Name()) != ".png" {
			continue
		}

		framePath := filepath.Join(sequencePath, frame.Name())
		canvas := mosaic.Canvas(framePath, util.DatabasePath, util.ResizeResolutions[spriteSizeIndex])
		mosaic.Draw(canvas, frame.Name(), util.ResizeResolutions[spriteSizeIndex])
	}

}
