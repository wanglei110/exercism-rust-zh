# 蛋白质转译

将 RNA 序列转译成蛋白质.

RNA 可以分解为三个称为密码子的核苷酸序列，然后转译成多肽，如下:

RNA:`"AUGUUUUCU"`=>转译成

密码子:`"AUG"， "UUU"， "UCU"`=>其成为具有以下序列的多肽=>

蛋白:`"Methionine"， "Phenylalanine"， "Serine"`

这有 64 个密码子，而这些密码子又相当于 20 个氨基酸；然而，在本练习中，所有密码子序列和所得氨基酸都不重要。如果它适用于一个密码子，该程序应该适用于所有这些密码子。但是，您可以随意扩展测试套件中的列表以包含它们.

还有三个终止密码子(也称为'STOP'密码子)；如果遇到任何这些密码子(通过核糖体)，那么所有转译结束，并终止蛋白质。

之后的所有后续密码子都会被忽略，如下所示:

RNA:`"AUGUUUUCUUAAAUG"`=>

密码:`"AUG"， "UUU"， "UCU"， "UAA"， "AUG"`=>

蛋白:`"Methionine"， "Phenylalanine"， "Serine"`

注意终止密码子`"UAA"`终止转译，最终的蛋氨酸，不会转译成蛋白质序列。

以下是本练习所需的密码子和产生的氨基酸。

| 密码子             | 蛋白     |
| :----------------- | :------- |
| AUG                | 蛋氨酸   |
| UUU，UUC           | 苯丙氨酸 |
| UUA，UUG           | 亮氨酸   |
| UCU，UCC，UCA，UCG | 丝氨酸   |
| UAU，UAC           | 酪氨酸   |
| UGU，UGC           | 半胱氨酸 |
| UGG                | 色氨酸   |
| UAA，UAG，UGA      | STOP     |

学习更多关于[蛋白质转译：维基百科](<http://en.wikipedia.org/wiki/Translation_(biology)>)

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/ch11-02-running-tests.html

## 资源

Tyler Long
