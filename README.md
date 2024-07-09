# octExtractor

I am excited to announce the release of octExtractor! A tool to extract metadata and images from binary OCT files built with Rust.
<p align="center">
  <img src="https://github.com/witedev/octExtractor/assets/159720725/4f56eb53-53fd-4426-9d49-9837e28c0afd" alt="thumbnail" width="200" />
  <img src="https://github.com/witedev/octExtractor/assets/159720725/a60bf954-7963-40cb-bc11-06f45a0ec337" alt="image" width="200" />
  <img src="https://github.com/witedev/octExtractor/assets/159720725/cefb1cb8-ae80-41a1-bebf-987e6c361665" alt="image" width="200" />
</p>

## Installation

- Download the pre-built binaries for your operating system from the [Releases](https://github.com/witedev/octExtractor/tree/main/releases) page. 
- Extract the downloaded file to a directory of your choice.

## Usage

To use octExtractor, run the following command:

```sh
octExtractor.exe <path_to_fda_file> -e extension -o output
./octExtractor <path_to_fda_file> -e extension
```

## Supported Output Extensions

- .png
- .tiff
- .jpg
- .bmp


## Command-Line Options

```sh
-o, --output <output_dir>: Specify the output directory for extracted files (default is the current directory).
-e, --extension <extension>: Specify the output file format (supported: png, tiff, jpg, bmp).
-h, --help
-v, --version
```

## Updates

9 July 2024
- [What's New in v0.1.0](https://github.com/witedev/octExtractor/releases/tag/v0.1.0) 

## Contributions
I want to give credit to this [inspiring repository](https://github.com/marksgraham/OCT-Converter/) for providing valuable ideas for this project.


##   License
This project is licensed under the MIT License. See the LICENSE file for details.

##  Contact
For any questions or suggestions, please open an issue on GitHub or contact me at [LinkedIn](https://www.linkedin.com/in/jes%C3%BAs-blanco-p%C3%A9rez-9843b2205/)






