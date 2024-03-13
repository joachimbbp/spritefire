package video

import (
	"fmt"
	"strconv"

	"github.com/joachimbbp/spritefire/src/util"
)

func BatchSequence(sequencePath string, spriteColorDbPath string, spriteResIndices []int) {

	fmt.Println(sequencePath, spriteColorDbPath, spriteResIndices)

	for _, resIndex := range spriteResIndices {

		res := strconv.Itoa(int(util.ResizeResolutions[resIndex]))
		util.ImageOutput = util.ImageOutputConst + "/" + res
		util.CreateIfNotExist(util.ImageOutput)
		fmt.Println("Resolution: ", res, "res index: ", resIndex)
		Sequence(sequencePath, spriteColorDbPath, resIndex)
	}
	util.ImageOutput = util.ImageOutputConst
}
