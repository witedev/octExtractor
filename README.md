# octExtractor

A tool to extract metadata and images from binary OCT files.

![thumbnail](https://github.com/witedev/octExtractor/assets/159720725/44cae964-5399-4bbc-9397-f76a9d4b41d2)

![image](https://github.com/witedev/octExtractor/assets/159720725/a60bf954-7963-40cb-bc11-06f45a0ec337)


## Overview

octExtractor is a high-performance tool written in Rust, designed to efficiently extract metadata and images from binary OCT (Optical Coherence Tomography) files. Its cross-platform compatibility ensures it works seamlessly on Windows, macOS, and Linux.

## Features

- **Fast and Efficient**: Built with Rust for optimal performance and speed.
- **Cross-Platform**: Compatible with Windows, macOS, and Linux.
- **Easy to Use**: Simple command-line interface for straightforward operation.
- **Flexible Output**: Extracts both metadata and images from OCT files.

## Installation

### Download Pre-built Binaries

1. Go to the [Releases](https://github.com/witedev/octExtractor/releases) page.
2. Download the appropriate binary for your operating system (Windows, macOS, or Linux).
3. Extract the downloaded file to a directory of your choice.

## Usage

To use octExtractor, run the following command:

```sh
octExtractor.exe <path_to_oct_file> -e extension -o output
./octExtractor <path_to_oct_file> -e extension -o output


