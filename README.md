# PRT: Predict, Resolve, Tally - The Rust Edition

A tool to record and score predictions about the future.

This is a somewhat overengineered version of [this Python version](https://github.com/Joern314/predict_resolve_tally) which was in turn inspired by [this bash script](https://github.com/NunoSempere/PredictResolveTally).

It was only tested on Linux, but should also work on Windows and MacOS.

## Installation
Install a rust toolchain and run `cargo install --path .` in this directory. 

- [ ]: Add a binary for linux.
- [ ]: Add AUR package

## Notes
This was written mostly as an attempt to overengineer the above linked Python version. I'd be happy to hear from you if you actually end up using it. 

It has no edit functionality on purpose. 
If you need to edit something, you can open the file where the predictions are stored directly, it is simple JSON. 
The data is stored under the standard "user data directories" for each OS: `$XDG_DATA_HOME/predict-resolve-tally/predictions.json` or `$HOME/.local/share/predict-resolve-tally/predictions.json` on Linux; `%appdata%\Roaming\predict-resolve-tally\data\predictions.json` on Windows and `$HOME/Library/Application Support/predict-resolve-tally/predictions.json` on MacOS.

