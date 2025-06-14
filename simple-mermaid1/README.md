# simple-mermaid1

`#[doc = simple_mermaid::mermaid!("path/to/mermaid")]`

`mermaid!` は `include_str!` と同様に相対パスでファイル名を指定する。

VS Code で `*.mmd` の preview には `vstirbu.vscode-mermaid-preview` などの拡張のインストールが必要になる。 <https://marketplace.visualstudio.com/items?itemName=vstirbu.vscode-mermaid-preview>

↑の拡張の場合は `Mermaid Preview: Preview Diagram` コマンドを実行することでプレビューを開くことができる。


`simple_mermaid::mermaid!` を入れておくと `cargo doc` でドキュメントを生成した場合に mermaid が埋め込まれる。
