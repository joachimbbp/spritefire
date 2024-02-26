#include "color.h"

#include <opencv2/opencv.hpp>

using namespace cv;

RGBColor getRGB(Point point, png::image<png::rgba_pixel> image)
{
    png::rgba_pixel pixel = image[point.y][point.x];
    return RGBColor{pixel.red, pixel.green, pixel.blue};
}

RGBAColor getRGBA(Point point, png::image<png::rgba_pixel> image)
{
    png::rgba_pixel pixel = image[point.y][point.x];
    return RGBAColor{
        RGBColor{pixel[2], pixel[1], pixel[0]},
        static_cast<float>(pixel[3])};
}
