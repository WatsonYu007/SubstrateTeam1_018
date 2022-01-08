# SubstrateTeam1_018

018-余祥龙-成都



第二次作业提交，请指教


老师，你好，我这边测试一直报错，一直解决不了，能帮我看看么，困扰我太久了，报错如下
```shell
running 2 tests
test mock::__construct_runtime_integrity_test::runtime_integrity_tests ... ok
test tests::create_kitty_works ... FAILED

failures:

---- tests::create_kitty_works stdout ----
thread 'tests::create_kitty_works' panicked at 'attempt to subtract with overflow', /rustc/cfa4ac66c194046f631ce076c75516ecfdeb77ee/library/core/src/ops/arith.rs:215:1
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::create_kitty_works

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

error: test failed, to rerun pass '-p pallet-template --lib'

```

以及

```shell
thread 'rustc' panicked at 'no entry found for key', compiler/rustc_metadata/src/rmeta/decoder.rs:1624:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.59.0-nightly (cfa4ac66c 2022-01-06) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [type_of] expanding type alias `service::FullClient`
#1 [fn_sig] computing function signature of `service::new_partial`
end of query stack
error: could not compile `node-template`
warning: build failed, waiting for other jobs to finish...
thread 'rustc' panicked at 'no entry found for key', compiler/rustc_metadata/src/rmeta/decoder.rs:1624:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.59.0-nightly (cfa4ac66c 2022-01-06) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [type_of] expanding type alias `service::FullClient`
#1 [fn_sig] computing function signature of `service::new_partial`
end of query stack
error: build failed
```
中途不知道啥原因，测试通过一次，之后就再也没有成功了
