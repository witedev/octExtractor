# octExtractor v0.1.0 Release

I am excited to announce the release of octExtractor v0.1.0! A tool to extract metadata and images from binary OCT files.
<p align="center">
  <img src="https://github.com/witedev/octExtractor/assets/159720725/4f56eb53-53fd-4426-9d49-9837e28c0afd" alt="thumbnail" width="200" />
  <img src="https://github.com/witedev/octExtractor/assets/159720725/a60bf954-7963-40cb-bc11-06f45a0ec337" alt="image" width="200" />
  <img src="https://github.com/witedev/octExtractor/assets/159720725/cefb1cb8-ae80-41a1-bebf-987e6c361665" alt="image" width="200" />
</p>

## What's New in v0.1.0

### Initial Release Highlights

- **File Support**: This initial version of octExtractor supports extraction from `.fda` files only.
- **High Performance**: Built with Rust, ensuring fast and efficient processing of OCT files.
- **Cross-Platform Compatibility**: Works seamlessly on Windows, macOS, and Linux.
- **Flexible Output**: Extract both metadata and images from OCT files with ease.
- **User-Friendly Interface**: Simple command-line interface for straightforward usage.

## Installation

Download the pre-built binaries for your operating system from the [Releases](https://github.com/witedev/octExtractor/releases) page. Extract the downloaded file to a directory of your choice.

## Usage

To use octExtractor, run the following command:

```sh
octExtractor.exe <path_to_fda_file> -e extension -o output
./octExtractor <path_to_fda_file> -e extension
```


## Supported Output Extensions

- png
- tiff
- jpg
- bmp


## Command-Line Options

```sh
-o, --output <output_dir>: Specify the output directory for extracted files (default is the current directory).
-e, --extension <extension>: Specify the output file format (supported: png, tiff, jpg, bmp).
-h, --help
-v, --version
```

## Usage

To use octExtractor, run the following command:

```sh
octExtractor.exe <path_to_fda_file> -e extension -o output
./octExtractor <path_to_fda_file> -e extension
```


## Examples

Extract data from an .fda file and save it as TIFF images in the default directory:

```sh
octExtractor.exe sample.fda -e tiff -o \path\to\output
./octExtractor sample.fda -e tiff -o /path/to/output
```

##  We welcome contributions! Please open an issue or submit a pull request on GitHub.

##   License
This project is licensed under the MIT License. See the LICENSE file for details.

##  Contact
For any questions or suggestions, please open an issue on GitHub or contact me at [LinkedIn](https://www.linkedin.com/in/jes%C3%BAs-blanco-p%C3%A9rez-9843b2205/)

## Credits
I want to give credit to the [inspiring repository](https://github.com/marksgraham/OCT-Converter/) for providing valuable ideas and inspiration for this project.




