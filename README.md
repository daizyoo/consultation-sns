# 相談用SNS

## 機能

- [ ] [User](#user)
  - [x] 作成
  - [x] 検索
  - [ ] 削除
- [ ] [相談](#article)
  - [x] 作成
  - [x] 検索
  - [ ] 削除
- [ ] [経験談](#article)
  - [x] 作成
  - [x] 検索
  - [ ] 削除
- [ ] [コメント](#comment)
  - [x] 作成
  - [ ] 検索
  - [ ] 削除
- [ ] タグ
- [ ] セッションID

## API

### User

---

#### api/user/create

|method|Type|
| :--: | :--: |
| POST | User|

---

#### api/user/search?

|method|
| :--: |
|GET(QueryParameter)|

|name|type|
| :--: | :--: |
|id|String|
|name|String|

### Article

---

#### api/article/post

|method|Type|
| :--: | :--: |
|POST| ArticlePost|

---

#### api/article/search?

|method|
| :--: |
|GET(QueryParameter)|

|name|type|
| :--: | :--: |
|<span style="color: red; ">type</span>|experience or consultation|
|logic| and or or|
|empathy|i16|
|nice|i16|
|title|String|
|text|String|

### Comment

---

#### api/comment/post

|method|Type|
| :--: | :--: |
| POST | CommentPost|

---

## Types

``` rust
struct User {
    id: String,
    name: String,
    password: String,
    introduction: Option<String>,
}

enum ArticleType {
    #[serde(rename(deserialize = "consultation"))]
    Consultation,
    #[serde(rename(deserialize = "experience"))]
    Experience,
}

struct Article {
    article_type: ArticleType,
    id: i16,
    user_id: String,
    empathy: i16,
    nice: i16,
    title: Option<String>,
    text: String,
}

struct Comment {
    id: i16,
    user_id: String,
    article_id: i16,
    empathy: i16,
    nice: i16,
    text: String,
}

struct PostUser<T = String> {
    id: T,
    name: T,
}

struct ArticlePost {
    article_type: ArticleType,
    user_id: String,
    title: Option<String>,
    text: String,
}

struct CommentPost {
    user_id: String,
    article_id: i16,
    text: String,
}

struct Response<T: Serialize> {
    status: bool,
    data: Option<T>,
}

struct SearchArticle {
    article_type: ArticleType,
    logic: Option<Logic>,
    empathy: Option<i16>,
    nice: Option<i16>,
    title: Option<String>,
    text: Option<String>,
}

enum Logic {
    #[serde(rename(deserialize = "and"))]
    And,
    #[serde(rename(deserialize = "or"))]
    Or,
}
```

## [SQL](migrations/20240326061154_migrate.sql)

## Files

[main](src/main.rs)

[User](src/user.rs)

[Article](src/article.rs)

[Comment](src/comment.rs)

[Type](src/types.rs)
