package util

type Rgb struct {
	R int
	G int
	B int
}

type IndexedSprite struct {
	Index  int
	Sprite string
}

func (c *Rgb) Get(i int) int {
	switch i {
	case 0:
		return c.R
	case 1:
		return c.G
	case 2:
		return c.B
	default:
		panic("Index out of range")
	}
}

func (c *Rgb) Print() {
	println(c.R, c.G, c.B)
}
