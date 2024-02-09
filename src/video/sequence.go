package video

import (
	"log"
	"os"
	"path/filepath"

	"github.com/joachimbbp/spritefire/src/mosaic"
	"github.com/joachimbbp/spritefire/src/util"
)

// takes in a sequence
func Sequence(sequencePath string, spriteColorDbPath string, spriteSizeIndex int) {
	//for every image in the sequence path run:
	//mosaic.Draw(mosaic.Canvas(util.InputStill, util.DatabasePath, util.ResizeResolutions[sRes])
	frames, err := os.ReadDir(sequencePath)
	if err != nil {
		log.Fatal(err)
	}
	for _, frame := range frames {
		if filepath.Ext(frame.Name()) != ".png" {
			continue
		}
		framePath := filepath.Join(sequencePath, frame.Name())
		mosaic.Draw(mosaic.Canvas(framePath, util.DatabasePath, util.ResizeResolutions[spriteSizeIndex]), frame.Name(), util.ResizeResolutions[spriteSizeIndex])
	}

}
