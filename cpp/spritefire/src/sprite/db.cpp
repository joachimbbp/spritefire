#include <filesystem>
#include <iostream>
#include <cstdio>
#include <map>
#include <opencv2/opencv.hpp>
#include "utils/color.h"

cv::Vec3i computeAverageColor(const cv::Mat &img)
{
    cv::Scalar sum = cv::sum(img); // should be the total of each value (if issues, check docs)
    double totalPixels = img.total();
    return cv::Vec3i(sum[0] / totalPixels, sum[1] / totalPixels, sum[2] / totalPixels);
}

RGBColor GetAverageColor(const std::string &imagePath)
{
    cv::Mat img = cv::imread(imagePath, cv::IMREAD_COLOR); // Q: so we pass in the address but this function takes the actual string? Something happens behind the scenes?
    cv::Vec3i averageColor = computeAverageColor(img);
    return RGBColor{
        averageColor[2], averageColor[1], averageColor[0] // reversed due to openCV BRG convention
    };
}

void PrintSpriteDatabase(const std::unordered_map<std::string, RGBColor> &spriteDb)
{
    // gpt written
    for (const auto &pair : spriteDb)
    {
        printf("File: %s, Average color: %d, %d, %d\n", pair.first.c_str(), pair.second.r, pair.second.g, pair.second.b);
    }
}

void CreateDatabase(const std::string &spriteFolderPath, const std::string &databaseSavePath)
{
    printf("Input folder: %s, Output destination: %s\n", spriteFolderPath, databaseSavePath);
    std::unordered_map<std::string, RGBColor> spriteDb;

    try
    {
        for (const auto &entry : std::filesystem::directory_iterator(spriteFolderPath))
        {
            spriteDb[entry.path().filename()] = GetAverageColor(entry.path());
        }
    }
    catch (std::filesystem::filesystem_error &e)
    {
        printf("%s", e.what());
    }
    PrintSpriteDatabase(spriteDb);
}