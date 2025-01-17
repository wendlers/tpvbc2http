# TPVBC2HTTP

Serve [TrainingPeaks Virtual](https://www.trainingpeaks.com/virtual/) data from [broadcast / streaming](https://help.trainingpeaks.com/hc/en-us/articles/31341004973453-TrainingPeaks-Virtual-Broadcast-Streaming-Mode) via HTTP. 

This project is to support TPVUI. For details and usage [see the project page](https://github.com/wendlers/tpvui).

This is in very early stage. No releases yet. But the project installs a set of github actions which will build _nightlies_ for _Windows_ and _Ubuntu_ on every push. You could download the artifacts for successful CI runs [here](https://github.com/wendlers/tpvbc2http/actions/workflows/rust.yml).

## Usage

[See TPVUI project page](https://github.com/wendlers/tpvui).

## Build / Run from Source

Make sure you are using the latest version of stable rust by running `rustup update`.

`cargo run --release`