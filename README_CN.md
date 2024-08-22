# WARPAutoSelect

[English](README.md) / 中文

## 简介

WARPAutoSelect 是一个简单的工具，它使用 [CloudflareWARPSpeedTest](https://github.com/peanut996/CloudflareWarpSpeedTest) 生成的结果自动选择最佳可用的WARP Endpoint。

## 用法

```bash
warp_auto_select -r result.csv -t 20
```

可用的选项:

- `-r`: `CloudflareWARPSpeedTest` 生成的结果文件，默认值为 `result.csv`。
- `-t`: 使用前 N 个 endpoint，默认值为 20。

## 例子

在使用此工具前，请确保可以使用 `warp-cli` 和 `CloudflareWARPSpeedTest` 命令。

1. 运行 `CloudflareWARPSpeedTest -o result.csv` 以生成结果文件。
2. 运行 `warp_auto_select -r result.csv` 以自动选择最佳 WARP Endpoint。

## License

此软件根据 [GPL v3 license](LICENSE) 发布。
