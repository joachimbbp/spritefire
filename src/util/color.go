package util

import (
	"image"
	"image/color"
	"math"
)

func GetRGBA(x int, y int, img image.Image) Rgba {
	rgbaColor := color.RGBAModel.Convert(img.At(x, y)).(color.RGBA)
	var rgba Rgba
	rgba.R = int(rgbaColor.R)
	rgba.G = int(rgbaColor.G)
	rgba.B = int(rgbaColor.B)
	rgba.A = int(rgbaColor.A)
	return rgba
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
