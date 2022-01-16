# SubstrateTeam1_018

018-余祥龙-成都



第三次作业提交，请指教

![测试结果](https://github.com/WatsonYu007/SubstrateTeam1_018/blob/kitties_ui/README.assets/Screenshot_from_2022-01-16%2011-51-17.png)

测试步骤：

1. 确保处于`kitties_ui`分支

2. 确认rust环境，执行`rustup show`命令，结果如下
    ```shell
    Default host: x86_64-unknown-linux-gnu
    rustup home:  /home/pick/.rustup

    installed toolchains
    --------------------

    stable-x86_64-unknown-linux-gnu
    nightly-x86_64-unknown-linux-gnu (default)

    installed targets for active toolchain
    --------------------------------------

    wasm32-unknown-unknown
    x86_64-unknown-linux-gnu

    active toolchain
    ----------------

    nightly-x86_64-unknown-linux-gnu (directory override for '/home/pick/Documents/xxx/SubstrateTeam1_018')
    rustc 1.59.0-nightly (cfa4ac66c 2022-01-06)
    ```
3. 编译节点，执行命令`cargo build --release`

4. 启动节点，执行命令`./target/release/node-template --dev --tmp`，如果数据显示不对，则先执行`./target/release/node-template purge-chain`

5. 确认node环境，执行如下命令
    ```shell
    pick@pc:~$ nvm --version
    0.39.1
    pick@pc:~$ node --version
    v14.18.3
    pick@pc:~$ nodejs --version
    v10.19.0
    ```

6. 进入substrate-front-end-template目录，执行命令`yarn install`以及`yarn start`
