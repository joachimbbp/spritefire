package util

import (
	"image"
	"image/color"
	"math"
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

func Distance(a Rgb, b Rgb) float64 {
	return math.Sqrt(float64((a.R-b.R)*(a.R-b.R) + (a.G-b.G)*(a.G-b.G) + (a.B-b.B)*(a.B-b.B)))
}
