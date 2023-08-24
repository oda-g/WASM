# WASM
WebAssembly(Wasm)について調査したメモ等を置いてある。

ブログ( https://valinux.hatenablog.com/entry/20230824 )の参考資料としても参照されている。ブログの方も参照されたい。

## 参照

以下の仕様書を参照している。

https://webassembly.github.io/spec/core/ (Release 2.0 (Draft 2023-06-14))

補足:   
常に最新版がおかれており、アクセスするとDraftの日付が異なっているかもしれない。
基本的には大きな違いはないはず。ドキュメントのソースは、以下にあるので、コミットログ
を参照すれば、違いが分かるはず。

https://github.com/WebAssembly/spec/

## 調査メモ

wikiにまとめているので、そちらを参照。

## プログラム

Wasmの勉強のため、モジュールの中身を表示したり、モジュールのごく簡単な実行を行うプログラムを作成した。wasmex/配下。

```
Usage: wasmex [OPTIONS] <PATH>

Arguments:
  <PATH>  path of wasm binary module

Options:
  -s          show section detail
  -d          show function code disassemble
  -i          interactive (typically to use to exec functions)
  -h, --help  Print help
```

- optionなし: モジュールに含まれるセクションとそのオブジェクト数のみ表示。
- s: セクションの内容を表示。
- d: 関数のコードをdisassembleした結果を表示。
- i: interactive。関数の実行に使用。まだ極僅かな命令しかサポートしておらず、作りかけ。

**注意**: 
- Rustに関しては初心者で勉強中なので、Rustプログラミングの観点では参考にならないと思う。
- 今後、Rustに対する理解が進むにつれ、コードは変更される可能性あり。
- Wasmの仕様理解の観点では、何某かの参考になると考える。
- 関数の実行に関しては、サポートする命令が今後増えていく予定。

**使用例**　　
```
$ cd wasmex
$ cargo run ../samples/sample.wasm -s
type[0]: (i32, i32) -> (i32)
func[0]: type=0
func[1]: type=0
export[0]: gcd func=0
code[0]: size(30) locals[]
code[1]: size(42) locals[i32]
```
(samplesについては今後作成予定)
