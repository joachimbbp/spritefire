package util

//possibly make this its own package?

import (
	"image"
	"image/color"
)

//structs etc relating to color

type Rgb struct {
	R int
	G int
	B int
}

func GetRGBA(x int, y int, img image.Image) color.RGBA {
	rgbaColor := color.RGBAModel.Convert(img.At(x, y)).(color.RGBA)
	return rgbaColor
}

func GetRGB(x int, y int, img image.Image) Rgb {
	rgbaColor := color.RGBAModel.Convert(img.At(x, y)).(color.RGBA)
	var rgb Rgb
	rgb.R = int(rgbaColor.R)
	rgb.G = int(rgbaColor.G)
	rgb.B = int(rgbaColor.B)
	return rgb
}
