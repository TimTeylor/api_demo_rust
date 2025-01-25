# api_demo_rust

Create lib:

```bash
cargo new --lib --vcs [version] [lib_name]
```

Create binary app

```bash
cargo new --bin --vcs [version] [bin_name]
```

Add external lib

```bash
cargo add -p [workspace] [lib_name]
```

Example creata preview image

```bash
cargo run -p cli_previewer -- -i [image_name] -w [width] --he [height] -f [filter]
```