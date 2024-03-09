package video

import (
	"fmt"
)

func BatchSequence(sequencePath string, spriteColorDbPath string, spriteResIndices []int) {
	fmt.Printf("not implemented yet, batchSequence")
	fmt.Println(sequencePath, spriteColorDbPath, spriteResIndices)

	for resIndex := range spriteResIndices {
		print((spriteResIndices[resIndex]))
		fmt.Println("")
		print(resIndex)

		fmt.Println("")
		Sequence(sequencePath, spriteColorDbPath, resIndex, true)
	}
}
