# 6. コンテキスト型

## 6.1 コンテキスト型の概念と目的

コンテキスト型は、暗黙的なコンテキスト渡しとリソースライフサイクル管理を統合するProtorun言語の重要な機能です。この機能は以下の目的で設計されています：

1. **暗黙的な依存性注入**: 明示的なパラメータ渡しなしで、関数やメソッドに必要なコンテキストを提供します。
2. **自動リソース管理**: スコープベースのリソースライフサイクル管理により、リソースの確実な解放を保証します。
3. **コードの簡潔さ**: 深いコールスタックを通じたコンテキスト伝播を簡素化し、コードの可読性を向上させます。
4. **関心の分離**: アプリケーションロジックとインフラストラクチャの懸念事項を分離します。

コンテキスト型は、依存性注入パターンの自然な実装を提供し、特定のスコープ内で暗黙的に利用可能なリソースを管理します。これにより、明示的なパラメータ渡しの煩雑さを軽減しつつ、型安全性とリソース管理の確実性を維持します。

## 6.2 コンテキスト型の定義

コンテキスト型は、`context type`キーワードを使用して定義されます。コンテキスト型は通常のデータ型と同様にフィールドとメソッドを持ちますが、特別なライフサイクル管理機能が追加されています。

```
// コンテキスト型の宣言
context type Database {
  // フィールド
  connection: DbConnection,
  
  // 初期化メソッド（ファクトリメソッド）
  fn connect(config: DbConfig): Result<Database, DbError> = {
    let conn = DbConnection.open(config)?
    Result.Ok(Database { connection: conn })
  }
  
  // 解放メソッド（自動的に呼び出される）
  fn close(self): Unit = {
    self.connection.close()
  }
  
  // 通常のメソッド
  fn query(self: &Self, sql: String): Result<QueryResult, DbError> = {
    self.connection.query(sql)
  }
}
```

コンテキスト型の定義には以下の特徴があります：

1. **初期化メソッド**: コンテキスト型のインスタンスを作成するファクトリメソッドを提供します。
2. **解放メソッド**: スコープ終了時に自動的に呼び出される解放メソッドを定義します。
3. **通常のメソッド**: コンテキスト型のインスタンスで使用できる通常のメソッドを定義します。

コンテキスト型は、管理型（managed type）と同様にリソースのライフサイクル管理を提供しますが、暗黙的なコンテキスト渡しの機能が追加されています。

## 6.3 コンテキスト型の使用

コンテキスト型は、`with`式を使用してスコープ内で暗黙的に利用可能になります。コンテキスト型を必要とする関数は、関数シグネチャに`with`キーワードを使用してその依存関係を宣言します。

```
// コンテキスト型を使用する関数
fn processUserData(userId: String): Result<UserData, Error> with Database = {
  // 暗黙的にコンテキストからDatabaseを使用
  let userData = Database.query(s"SELECT * FROM users WHERE id = $userId")?
  Result.Ok(parseUserData(userData))
}

// コンテキスト型の提供
fn main(): Result<Unit, Error> = {
  // データベース接続を確立
  let db = Database.connect(config)?
  
  // コンテキスト型として提供
  with db {
    // dbが暗黙的に利用可能になる
    let userData = processUserData("user123")?
    displayUserData(userData)
    
    Result.Ok(())
  } // dbは自動的に閉じられる（close()メソッドが呼び出される）
}
```

コンテキスト型の使用には以下の特徴があります：

1. **暗黙的な依存関係**: 関数は`with`キーワードを使用して必要なコンテキスト型を宣言します。
2. **スコープベースの提供**: `with`式を使用してコンテキスト型をスコープ内で提供します。
3. **自動リソース管理**: スコープ終了時にコンテキスト型の解放メソッドが自動的に呼び出されます。
4. **型安全性**: コンパイラは必要なコンテキスト型が提供されていることを静的に検証します。

## 6.4 複数のコンテキスト型

複数のコンテキスト型を同時に使用することができます。これにより、異なる種類のリソースやサービスを組み合わせて使用できます。

```
// 複数のコンテキスト型を使用する関数
fn processUserOrder(userId: String, orderId: String): Result<OrderDetails, Error> with Database & Logger & HttpClient = {
  Logger.info(s"処理開始: ユーザー $userId, 注文 $orderId")
  
  let userData = Database.query(s"SELECT * FROM users WHERE id = $userId")?
  let orderData = Database.query(s"SELECT * FROM orders WHERE id = $orderId")?
  
  let additionalInfo = HttpClient.get(s"https://api.example.com/orders/$orderId/details")?
  
  Logger.info("処理完了")
  Result.Ok(combineOrderDetails(userData, orderData, additionalInfo))
}

// 複数のコンテキスト型を提供
fn main(): Result<Unit, Error> = {
  let db = Database.connect(dbConfig)?
  let logger = Logger.init(logConfig)
  let client = HttpClient.create(httpConfig)
  
  with db {
    with logger {
      with client {
        // すべてのコンテキスト型が利用可能
        let orderDetails = processUserOrder("user123", "order456")?
        displayOrderDetails(orderDetails)
        
        Result.Ok(())
      } // clientは自動的に閉じられる
    } // loggerは自動的に閉じられる
  } // dbは自動的に閉じられる
}
```

ネストされた`with`式は、以下のように簡略化することもできます：

```
with db, logger, client {
  // すべてのコンテキスト型が利用可能
  let orderDetails = processUserOrder("user123", "order456")?
  displayOrderDetails(orderDetails)
  
  Result.Ok(())
} // すべてのコンテキストは自動的に閉じられる
```

## 6.5 コンテキスト型の合成

コンテキスト型は合成することができ、より大きなコンテキストを構築できます。これにより、関連するリソースやサービスをグループ化し、一緒に管理することができます。

```
// 基本的なコンテキスト型
context type Database { /* ... */ }
context type Logger { /* ... */ }

// 合成されたコンテキスト型
context type AppContext {
  db: Database,
  logger: Logger,
  
  // 初期化
  fn create(dbConfig: DbConfig, logConfig: LogConfig): Result<AppContext, Error> = {
    let db = Database.connect(dbConfig)?
    let logger = Logger.init(logConfig)
    Result.Ok(AppContext { db, logger })
  }
  
  // 解放（自動的に呼び出される）
  fn close(self): Unit = {
    self.logger.close()
    self.db.close()
  }
  
  // 委譲メソッド
  fn query(self: &Self, sql: String): Result<QueryResult, DbError> = {
    self.db.query(sql)
  }
  
  fn log(self: &Self, message: String): Unit = {
    self.logger.log(message)
  }
}

// 使用例
fn processData(): Result<Unit, Error> with AppContext = {
  AppContext.log("処理開始")
  let result = AppContext.query("SELECT * FROM data")?
  // 処理...
  AppContext.log("処理完了")
  Result.Ok(())
}
```

## 6.6 コンテキスト型と代数的効果の関係

コンテキスト型と代数的効果は、異なる問題を解決するために設計された補完的な概念です：

1. **コンテキスト型**: 暗黙的なコンテキスト渡しとリソースライフサイクル管理に焦点を当てています。
2. **代数的効果**: 副作用の型安全な表現と制御に焦点を当てています。

これらは以下のように連携することができます：

```
// 代数的効果を使用してコンテキスト型にアクセス
effect DbAccess {
  fn query(sql: String): Result<QueryResult, DbError>
}

// コンテキスト型を使用して効果を実装
fn runWithDatabase<T>(db: Database, action: () -> T with DbAccess): T = {
  handler DbHandler for DbAccess {
    fn query(sql: String): Result<QueryResult, DbError> = {
      db.query(sql)
    }
  }
  
  with DbAccess handled by DbHandler {
    action()
  }
}

// 使用例
fn main(): Result<Unit, Error> = {
  let db = Database.connect(config)?
  
  with db {
    let result = runWithDatabase(db, () => {
      // DbAccess効果を使用
      let data = DbAccess.query("SELECT * FROM users")?
      processData(data)
    })
    
    Result.Ok(())
  }
}
```

この組み合わせにより、コンテキスト型のリソース管理と代数的効果の副作用制御の両方の利点を活用できます。

## 6.7 設計上の考慮事項

コンテキスト型を設計する際には、以下の点を考慮することが重要です：

1. **粒度**: コンテキスト型は適切な粒度で設計し、関連する機能をグループ化します。
2. **ライフサイクル**: コンテキスト型のライフサイクルを明確に定義し、適切な初期化と解放を保証します。
3. **合成**: コンテキスト型は合成可能に設計し、より大きなコンテキストを構築できるようにします。
4. **インターフェース**: コンテキスト型は明確で一貫したインターフェースを提供し、使用しやすくします。
5. **テスト容易性**: コンテキスト型はモック可能に設計し、テストを容易にします。

コンテキスト型は、依存性注入と自動リソース管理の強力な組み合わせを提供し、クリーンで保守性の高いコードを作成するための重要なツールです。
