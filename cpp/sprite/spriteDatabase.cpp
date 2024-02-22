//for now we will write without the png++ integration
//Pierre is working on getting that going
//To test, let's just start with building the directory
//and looping through everything within the subfolder


#include <filesystem>
#include <iostream>
#include <cstdio>
using namespace std;


struct RGB{ //eventually this will live in the library
    int r;
    int g;
    int b;
};

RGB averageColor(const string& imagePath);


void database(const string& input, const string& output){
printf("Input folder: %s, Output destination: %s\n", input, output);
unordered_map<string, RGB> spriteColorAverages;

try{
    for (const auto & entry :filesystem::directory_iterator(input)){
        //to debug, this prints out the filename
        string spriteName = entry.path().filename();
        cout << spriteName << endl;
        spriteColorAverages[spriteName] = averageColor(entry.path());
    }
}
catch (filesystem::filesystem_error& e){
    cout << e.what() << '\n';
}

}

RGB averageColor(const string& imagePath){//this could be a universal function tbh
cout<<"this isn't implemented yet, just adding black rgb val"<<endl;
//waiting to plug this into the png++ or openframeworks library...
return {0,0,0};
}