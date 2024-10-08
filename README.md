# WARPAutoSelect

English / [中文](README_CN.md)

## Introduction

WARPAutoSelect is a simple tool that automatically selects the best available WARP endpoint using result generated by [CloudflareWARPSpeedTest](https://github.com/peanut996/CloudflareWarpSpeedTest).

## Usage

```bash
warp_auto_select -r result.csv -t 20
```

Available options:

- `-r`: The result file generated by `CloudflareWARPSpeedTest`, default is `result.csv`.
- `-t`: Use top N endpoint, default is 20.

## Example

Make sure that `warp-cli` and `CloudflareWARPSpeedTest` is available before running this tool.

1. Run `CloudflareWARPSpeedTest -o result.csv` to generate a result file.
2. Run `warp_auto_select -r result.csv` to select the best WARP endpoint automatically.

## License

This software is released under the [GPL v3 license](LICENSE).
