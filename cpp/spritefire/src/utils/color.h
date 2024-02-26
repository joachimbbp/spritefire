#pragma once

#include <opencv2/opencv.hpp>

using namespace cv;

struct RGBColor {
    int r, g, b;
};

struct RGBAColor {
    RGBColor rgb;
    float a;
};

RGBColor getRGB(Point point, Mat& image);
RGBAColor getRGBA(Point point, Mat& image);
