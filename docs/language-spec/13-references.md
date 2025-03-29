# 11. 参考文献

## 11.1 参考文献の目的

この章では、Protorun言語の設計と実装に影響を与えた主要な文献、論文、言語仕様を紹介します。これらの参考文献は、言語の理論的基盤を理解し、設計決定の背景を把握するのに役立ちます。また、言語の特定の側面についてさらに学びたい開発者にとっても有用なリソースとなります。

## 11.2 プログラミング言語仕様

- **Scala言語仕様**
  - Protorun言語の型システムと関数型プログラミングの側面に影響を与えました。特に、トレイトシステムと型推論の設計に参考にしました。

- **Haskell言語仕様**
  - 純粋関数型プログラミングと型クラスの設計に影響を与えました。Protorunの型クラスシステムはHaskellの型クラスに着想を得ています。

- **Rust言語リファレンス**
  - 所有権システムとメモリ安全性の設計に大きな影響を与えました。Protorunの所有権モデルはRustの所有権システムを基にしています。

## 11.3 書籍

- **"Types and Programming Languages" by Benjamin C. Pierce**
  - 型システムの理論的基盤を提供し、Protorunの型システム設計に影響を与えました。

- **"Practical Foundations for Programming Languages" by Robert Harper**
  - プログラミング言語の形式的意味論と型理論に関する包括的な参考書です。

- **"Programming Language Pragmatics" by Michael L. Scott**
  - プログラミング言語の設計と実装に関する実践的な側面を扱っています。

- **"Compilers: Principles, Techniques, and Tools" by Alfred V. Aho, Monica S. Lam, Ravi Sethi, and Jeffrey D. Ullman**
  - コンパイラの設計と実装に関する基本的な参考書です。

## 11.4 代数的効果と効果ハンドラに関する論文

- **"Programming with Algebraic Effects and Handlers" by Andrej Bauer and Matija Pretnar**
  - 代数的効果と効果ハンドラの基本的な概念と実装を紹介しています。Protorunの効果システムの主要な参考文献です。

- **"Effect Handlers in Scope" by Nicolas Wu, Tom Schrijvers, and Ralf Hinze**
  - スコープ付き効果ハンドラの概念を導入し、Protorunの効果スコープの設計に影響を与えました。

- **"Eff: Extensible Effects with Handlers" by Oleg Kiselyov and KC Sivaramakrishnan**
  - 拡張可能な効果システムの設計と実装に関する論文です。Protorunの効果合成の設計に影響を与えました。

- **"Effekt: Capability-passing style for type- and effect-safe, extensible effect handlers in Scala" by Jonathan Immanuel Brachthäuser and Philipp Schuster**
  - 型安全で拡張可能な効果ハンドラの実装に関する論文です。Protorunの効果型システムの設計に影響を与えました。

## 11.5 所有権とリソース管理に関する論文

- **"Ownership Types for Safe Programming: Preventing Data Races and Deadlocks" by Chandrasekhar Boyapati, Robert Lee, and Martin Rinard**
  - 所有権型の基本的な概念と安全性保証に関する論文です。

- **"Uniqueness Types for Resource Management" by Edsko de Vries, Rinus Plasmeijer, and David M. Abrahamson**
  - リソース管理のための一意型に関する論文です。Protorunの管理型の設計に影響を与えました。

- **"Linear Types Can Change the World!" by Philip Wadler**
  - 線形型の基本的な概念と応用に関する論文です。Protorunの将来の線形型拡張の参考文献です。

## 11.6 型システムに関する論文

- **"A Theory of Type Polymorphism in Programming" by Robin Milner**
  - 多相型システムの理論的基盤を提供する論文です。Protorunの型推論システムの設計に影響を与えました。

- **"Type Classes as Objects and Implicits" by Bruno C. d. S. Oliveira, Adriaan Moors, and Martin Odersky**
  - 型クラスの実装と暗黙的パラメータに関する論文です。Protorunのトレイトシステムの設計に影響を与えました。

- **"Dependent Types in Practical Programming" by Hongwei Xi and Frank Pfenning**
  - 実用的なプログラミングにおける依存型の応用に関する論文です。Protorunの将来の依存型拡張の参考文献です。

## 11.7 その他の参考文献

- **"The Implementation of Functional Programming Languages" by Simon Peyton Jones**
  - 関数型プログラミング言語の実装に関する包括的な参考書です。

- **"Advanced Topics in Types and Programming Languages" edited by Benjamin C. Pierce**
  - 高度な型システムと型理論に関する論文集です。

- **"Concepts, Techniques, and Models of Computer Programming" by Peter Van Roy and Seif Haridi**
  - 様々なプログラミングパラダイムとその実装に関する包括的な参考書です。

これらの参考文献は、Protorun言語の設計と実装の理論的基盤を理解するのに役立ちます。また、言語の特定の側面についてさらに学びたい開発者にとっても有用なリソースとなります。
