# 并发字母频率

使用并行计算，文本中的字母频率。

并行性是并行的，也可以按顺序进行。一个常见的例子是计算字母的频率。创建一个函数，返回文本列表中每个字母的总频率，并使用并行性。

# Rust 中的并发性字母频率

在这里了解更多关于 Rust 的并发性:

- [Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)

## 加分

这个练习还包括一个基准，以一个顺序实现为基线。您可以将您的解决方案与基准进行比较。观察不同大小的输入，对每个性能的影响。可以使用并行编程技术。来超越基准吗?

在本文中，`test::Bencher` 是不稳定的，只能在*nightly*Rust 可用。用 Cargo 运行基准:

```bash
cargo bench
```

如果你使用 rustup.rs:

```
rustup run nightly cargo bench
```

- [基准测试](https://doc.rust-lang.org/stable/unstable-book/library-features/test.html)

了解 nightly Rust 的更多信息:

- [Nightly Rust](https://doc.rust-lang.org/stable/book/2018-edition/appendix-06-nightly-rust.html)
- [Rustup: Working with nightly](https://github.com/rust-lang-nursery/rustup.rs#working-with-nightly-rust)

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/ch11-02-running-tests.html
