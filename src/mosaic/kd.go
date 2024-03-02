package mosaic

import (
	"math"

	"github.com/joachimbbp/spritefire/src/util"
)

type Node struct {
	color util.Rgb
	left  *Node
	right *Node
}

type KDTree struct {
	root *Node
}

func InsertInTree(current *Node, color util.Rgb, depth int) *Node {
	if current == nil {
		return &Node{color: color, left: nil, right: nil}
	}

	axis := depth % 3

	if current.color.Get(axis) < color.Get(axis) {
		current.right = InsertInTree(current.right, color, depth+1)
	} else {
		current.left = InsertInTree(current.left, color, depth+1)
	}

	return current
}

func SearchInTree(current *Node, query util.Rgb, depth int) *Node {
	if current == nil {
		return nil
	}

	isEqual := true
	for axis := 0; axis < 3; axis++ {
		if current.color.Get(axis) != query.Get(axis) {
			isEqual = false
			break
		}
	}

	if isEqual {
		return current
	}

	axis := depth % 3

	if current.color.Get(axis) < query.Get(axis) {
		return SearchInTree(current.right, query, depth+1)
	}
	return SearchInTree(current.left, query, depth+1)
}

func FindNearestNeighborInTree(current *Node, query util.Rgb, best *Node, depth int) *Node {
	if current == nil {
		return best
	}

	if best == nil || util.Distance(query, current.color) < util.Distance(query, best.color) {
		best = current
	}

	axis := depth % 3

	var nextBranch *Node
	var oppositeBranch *Node

	// Decide which way to go: left or right
	if query.Get(axis) < current.color.Get(axis) {
		nextBranch = current.left
		oppositeBranch = current.right
	} else {
		nextBranch = current.right
		oppositeBranch = current.left
	}

	best = FindNearestNeighborInTree(nextBranch, query, best, depth+1)

	// Backtrack if there is a chance that the opposite branch could contain a closer point
	radius := util.Distance(query, best.color)
	dist := math.Abs(float64(query.Get(axis) - current.color.Get(axis)))

	if radius > dist {
		best = FindNearestNeighborInTree(oppositeBranch, query, best, depth+1)
	}

	return best
}

func TraverseTree(current *Node) {
	if current == nil {
		return
	}

	current.color.Print()
	TraverseTree(current.left)
	TraverseTree(current.right)
}

func (tree *KDTree) Insert(color util.Rgb) {
	tree.root = InsertInTree(tree.root, color, 0)
}

func (tree *KDTree) Search(query util.Rgb) *Node {
	return SearchInTree(tree.root, query, 0)
}

func (tree *KDTree) FindNearestNeighbor(query util.Rgb) *Node {
	return FindNearestNeighborInTree(tree.root, query, nil, 0)
}

func (tree *KDTree) Traverse() {
	TraverseTree(tree.root)
}
