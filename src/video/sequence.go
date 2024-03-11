package video

import (
	"log"
	"os"
	"path/filepath"
	"sort"
	"strconv"
	"sync"

	"github.com/joachimbbp/spritefire/src/mosaic"
	"github.com/joachimbbp/spritefire/src/search"
	"github.com/joachimbbp/spritefire/src/util"
)

func Sequence(sequencePath string, spriteColorDbPath string, spriteSizeIndex int) {
	prefix := strconv.Itoa((util.ResizeResolutions[spriteSizeIndex])) + "_"

	frames, err := os.ReadDir(sequencePath)
	if err != nil {
		log.Fatal(err)
	}

	sequenceData := make(map[string][]util.IndexedSprite)
	spriteColorDb := util.DecodeColorDatabase(spriteColorDbPath)
	tree := search.BuildSearchTree(spriteColorDb)

	var wg sync.WaitGroup
	var mu sync.Mutex

	for _, frame := range frames {
		if filepath.Ext(frame.Name()) != ".png" {
			continue
		}
		wg.Add(1)
		go func(frame os.DirEntry) {
			defer wg.Done()
			framePath := filepath.Join(sequencePath, frame.Name())
			spriteCanvas := mosaic.Canvas(framePath, spriteColorDb, spriteSizeIndex, tree)
			mu.Lock()
			sequenceData[frame.Name()] = spriteCanvas

			mu.Unlock()
		}(frame)
	}
	wg.Wait()

	keys := make([]string, 0, len(sequenceData))
	for k := range sequenceData {
		keys = append(keys, k)
	}
	sort.Strings(keys)

	for _, frameName := range keys {
		mosaic.Draw(sequenceData[frameName], prefix, frameName, spriteSizeIndex)

	}

}
