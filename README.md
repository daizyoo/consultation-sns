# SNS

## - _ -

相談用のSNS

## 機能

### 投稿

- 経験談
  - コメント
  - [タグ](#タグ)
- 相談
  - 回答
  - [タグ](#タグ)
- チャット(仮)

- [レッテル](#レッテル)

### タグ

### レッテル

複数人が貼ったら表示

### ~~荒らし対策~~

- 特定のタグがついてる人にしかDMできない
- 投稿者は返信できるユーザーをタグで制限できる
- ログインしていないユーザーは閲覧のみ

## 実装

### 経験談

- [home](app/pages/experience/index.html)

### [相談](app/pages/consultation/index.html)

### [ユーザー](app/pages/user/index.html)

## API

- user
  - login
  - create
  - {id}
- experience
  - post
- consultation
  - post
