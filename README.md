# Drawio-rs

Crate for drawing graphics using [draw.io]. It is intended to be simple to use, maintain and extent.

Main focus is on to provide a convenient way to draw common diagrams, like bar charts, stacked bar charts and pie charts. However, you can use and extend the library to support other diagrams or models supported by [draw.io].

Use cases:

- Direct use, [draw.io] allows to export your diagrams in various formats, jpg, pdf, png, svg, vsdx, and xml. You can export in asciidoc format using the [drawio-exporter] tool.
- Editing use. [draw.io] is an excellent tool for graphical layout and diagrams with integration Teams and cloud storage for collaborative work. You can use this tool/library to generate diagrams from raw data, later to be included and refined.
- When other diagrams fall short (not providing the type of diagrams you want, or not the flexibility need for your use). You can use the primitives provided by this library to device custom diagrams only limited by the capabilities of [draw.io]. Alternatively, you can fork the repository and contribute with your own diagrams.

There are plenty of alternatives for generating diagrams, to name a few with Rust support.

- [plotters](https://crates.io/crates/plotters) Native plotting library, with a lot of functionality.
- [plotlib](https://crates.io/crates/plotlib) Native plotting library.
- [gnuplot](https://crates.io/crates/gnuplot) Non-native depends on the `gnuplot` tool. Renders `gnuplot` data files, from which you can generate diagrams, and manipulate with scripts.
- [plotly](https://crates.io/crates/plotly) Non-native, depends on the `plotly` python ecosystem.

Other alternatives include [Graphviz](https://graphviz.org/), which focus on highly scalable automatic graph layout (there exists various Rust based tools for interacting with `graphviz`, e.g., [graphviz-rust](https://crates.io/crates/graphviz-rust), and even [graphviz-sys](https://crates.io/crates/graphviz-sys) library bindings).

For Markdown integration Mermaid provides a set of simple to use diagrams. Good support for native rendering in Markdown, see e.g., [mermaid aquamarine](https://crates.io/crates/aquamarine), however Mermaid diagrams are rather limited in flexibility.

---

## Local Installation of [draw.io]

This is not strictly required you may use [draw.io online]. The advantage of having it installed locally is that you can edit the generated diagrams directly and even build your own tools that automatically render your diagrams in various formats.

First install [draw.io] locally on your computer and make it available from terminal.

### Native Windows

- Install the program [draw.io].
- Add to path for the shell you use, e.g., in PowerShell `$Env:Path += ';<PATH>`, where `<PATH>` is the full path to the folder where the `draw.io.exe` is stored - in the case of a system wide install:
  
  ```shell
  $Env:Path += ';C:\Program Files\draw.io'
  ```

### Linux, Osx and Windows WSL

- Install [draw.io] using your package manager, should be in path by default.

---

## Use

- `.drawio` files are stored in the `xml` folder.
- The `draw.io` program can be used to view/edit `.drawio` files. It can also be used to export in `pdf`, `svg` etc. This is also possible in batch (command line) mode, e.g., a (cropped) `pdf` is generated by:

  ```shell
  draw.io -x -f pdf -o out.pdf --crop .\xml\out.drawio
  ```
  
  Batching will eventually be supported by the library but for now you can run the command in a shell.

---

## Design

The [draw.io] stores models in a fairly simple `xml` format. While there exists numerous `xml` support crates for handling `dom` structures we provide a light weight `xlm.rs` library with a builder pattern, for creating and manipulating tags, attributes and styles.

Drawing primitives are found in the `draw.rs` file, which adds `draw.io` specific functionality.

Based on that, `bar_chart.rs`, `stacked_bar_chart.rs`, and `pie_chart.rs` can be used to generate diagrams accordingly (unit tests covers examples of use).

--

## Testing

You can run individual tests, e.g., the `test_export` and view output by:

```shell
cargo test test_export -- --nocapture
```

--

## References

- [draw.io] For local download and documentation.
- [draw.io online] For online use. If you don't want
- [drawio-exporter] Cargo sub-command for command line exports to various formats including asciidoc.

[draw.io online]: https://app.diagrams.net/
[draw.io]: https://www.drawio.com/
[drawio-exporter]: https://crates.io/crates/drawio-exporter
