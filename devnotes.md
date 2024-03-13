# KNOWN BUGS
- Program crashes towards the end of large batch jobs
    - Once this is fixed, have the CI run a batch (not just video)

# PULL REQUESTS AND CONTRIBUTERS
- feel free to contact me at jbbpfefferkorn@gmail.com for any questions

# WISH LIST
- Better CI tools which work with raylib
- Raylib sprite resize
- Goroutine database creation
- Fix issue with crashing while processing large batches (see known bugs)
- Generate docs

# BLOG:
March 01 2024:
Mishima took 3051757667834%  (half an hour)
    879 frames, sprite size index 5 (30x30)

March 05 2024:
KD trees implemented
Mishima took generate video took 855834208334% (eight minutes)
    879 frames, sprite size index 5 (30x30)
    Small glitch in bottom right corner persists: no sprites are rendered there
    Raylib does not appear to be outputing alpha layer
    KD tree matches slightly different

March 07 2024:
Refactoring for useability
    database creation took 26 seconds
        This still needs to be multithreaded, it's quite slow
    resizing took 0 hours 2 minutes 22 seconds
    generate video took 0 hours 1 minutes 33 seconds
        This is for the new "scuba" test video at resolution 5

March 08 2024:
Dogfooding the app with my own video.
    It feels like compositing the sprites on with alpha only sometimes creates compelling results.
    I'm getting the handle on what kind of images look and feel best
Introduced some restrictions on the goroutines (from copilot). I need to brush up on the logic with these, but the purpose is to keep the program for crashing with the new batch feature. Currently it doesn't seem to restrict performance: generating video took 0 hours 1 minutes 34 seconds with this new 10 core restriction.

March 11 2024:
    Was pretty tired on friday so made some less than stellar decisions with the batch processing. Refactored today and it's much better.
    However, there seems to be a memory leak (or something) that causes the program to crash after processing many sprites, especially at a higher "reslolution" (meaning, confusingly, a lower sprite resolution, thus more sprites per image)
    Adding After Effects switcher.
    Upon further investigation, it does not appear to be a memory leak. It just crashes at some point.

    Github actions do not take in raylib, so built out an offline testing case in main which runs the whole project from no resized sprites or database to a final image.
    Total Time for full offline test:  4.360724434016666 minutes

    Calling it for now as I want to go work on some other projects but there's still some work to be done (see "next features" section)

    Actually, did a bit more work on this. Once compiled, the program seems to be much more stable. Offline tests went great. Total Time for full offline test:  21.236245025 minutes
    Moving forward I will use smaller clips for dogfooding and always with a compiled executable until I can get the code more stable.

March 12 2024:
    I've been testing the app on my own footage for a few days now. Found many bugs and instabilities, but it seems like everything runs best when compiled, not directly from main with `go run`. Smaller sequences and smaller jobs are -predictably- more stable as well. I think it's time to ge the readme in order and ready for a public-facing repo.
    The program crashed on the last resolution with offline tests while running from vscode's terminal, but ran well when running from a compiled file in the terminal. Total runtime for full offline test: 21 minutes on a 2023 M2 macbook pro, 14 inches, 16GB ram.