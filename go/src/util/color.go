package util

import (
	"image"
	"image/color"
)

func GetRGBA(x int, y int, img image.Image) color.RGBA {
	rgbaColor := color.RGBAModel.Convert(img.At(x, y)).(color.RGBA)
	return rgbaColor
}

// Custom Rgb type can be found in customStruct.go
func GetRGB(x int, y int, img image.Image) Rgb {
	rgbaColor := color.RGBAModel.Convert(img.At(x, y)).(color.RGBA)
	var rgb Rgb
	rgb.R = int(rgbaColor.R)
	rgb.G = int(rgbaColor.G)
	rgb.B = int(rgbaColor.B)
	return rgb
}
