// Perhaps when this runs in parallel in GLSL this won't be needed?
#include <std>            //ugh how do you get rid of these red squiggles
#include <opencv/opencv4> //but WHICH HPP????? //also, perhaps select only the necessary libraries when importing?
#include "utils/resolution.h"
#include <filesystem>
#include <iostream>

std::string buildFolder(const int resolution, const std::string &resizedSavePath) // need pointer help here...
{
    std::ostringstream os;
    os << resizedSavePath << resolution;
    std::filesystem::create_dictionary(os.str());
    return os.str()
}

void resizeSprite(std::string inputImagePath, const int *targetResolution, const std::string &outputFolderPath)
{
    std::string spriteName = path(inputImagePath).filename().string() cv::Mat img = cv::imread(inputImagePath, cv::IMREAD_COLOR);
    if (img.empty())
    {
        std::cerr << "Could not open or find the image" << std::endl;
        return;
    }
    // width and height are the same as we are dealing with square sprites
    cv::resize(img, img, cv::Size(targetResolution, targetResolution));
    // build output path
    std::string outputPath = outputFolderPath + "/" + spriteName;

    cv::imwrite(outputPath, img)
}

void resizeSprites(const std::string &spriteSourceFolderPath, const std::string &resizedSavePath)
{
    printf("Resizing Sprites\n");
    for (const int resolution : RESIZE_RESOLUTIONS)
    {
        std::string resFolderPath = buildFolder(resolution, &resizedSavePath);
        for (const auto &sprite : std::filesystem::directory_iterator(spriteSourceFolderPath)) // possible type mismatch
        // is this going to give an image?
        {
            resizeSprite(&sprite, resolution, resFolderPath)
        }
    }
}