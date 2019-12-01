# File Fixer
The purpose of this project is to format names to be compliant on a Windows system. Take for example `file.jpg:large.jpg`. This file won't open correctly on Windows, though it will work fine on Linux. This project will turn that file into something like this `file.jpg_large.jpg` which should be compliant.

Rules followed are listed [here](https://docs.microsoft.com/en-us/windows/win32/fileio/naming-a-file). I have added an additional rule of no spaces allowed, despite spaces being allowed on both Windows and Linux. This is just a ease-of-use rule.

## Usage
```
cargo run <path-to-files>
```

A file path can be provided, while leaving it empty will run against the current directory.