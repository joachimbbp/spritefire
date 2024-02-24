#include "color.h"

#include <opencv2/opencv.hpp>

using namespace cv;

RGBColor getRGB(Point point, Mat& image) {
    Vec3b pixel = image.at<Vec3b>(point.y, point.x);
    return RGBColor{pixel[2], pixel[1], pixel[0]};
}

RGBAColor getRGBA(Point point, Mat& image) {
    Vec4b pixel = image.at<Vec4b>(point.y, point.x);
    return RGBAColor{
        RGBColor{pixel[2], pixel[1], pixel[0]},
        static_cast<float>(pixel[3])};
}
