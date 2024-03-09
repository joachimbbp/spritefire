package video

import (
	"fmt"

	"github.com/joachimbbp/spritefire/src/util"
)

func BatchSequence(sequencePath string, spriteColorDbPath string, spriteResIndices []int) {
	fmt.Printf("not implemented yet, batchSequence")
	fmt.Println(sequencePath, spriteColorDbPath, spriteResIndices)

	for _, resIndex := range spriteResIndices {
		fmt.Println("resolution: ")
		fmt.Println((util.SpriteSizes[resIndex]))

		fmt.Println("index: ")
		fmt.Println(resIndex)

		//Sequence(sequencePath, spriteColorDbPath, resIndex, true)
	}
}
