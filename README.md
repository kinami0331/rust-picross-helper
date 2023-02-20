# rust-picross-helper

基于 rust 的 picross 游戏求解器，预计支持编译为 wasm 及 python 的库。

摸鱼时写着玩的，不一定能写完（x

目前状态：新建文件夹

## 部署

### 编译为 `wasm`

1. 安装 [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
2. 在 `game_lib_wasm` 目录中 `wasm-pack build --target=web`
3. `demo_html` 下有一个 `demo.html`，其中包含了调用编译好的 `wasm` 文件的一个简单示例。
