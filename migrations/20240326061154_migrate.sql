CREATE TABLE IF NOT EXISTS users (
    id VARCHAR(15) NOT NULL,
    name VARCHAR(30) NOT NULL,
    password VARCHAR(20) NOT NULL,
    introduction VARCHAR(150),
    PRIMARY KEY (id)
);

COMMENT ON COLUMN users.introduction IS '自己紹介';

CREATE TABLE comment(
    id SERIAL NOT NULL,
    user_id varchar(15) NOT NULL,
    article_id SERIAL NOT NULL,
    empathy smallint NOT NULL DEFAULT 0,
    nice smallint NOT NULL DEFAULT 0,
    text varchar(300) NOT NULL,
    PRIMARY KEY(id),
    CONSTRAINT comment_user_id_fkey FOREIGN key(user_id) REFERENCES users(id),
    CONSTRAINT comment_article_id_fkey FOREIGN key(article_id) REFERENCES article(id)
);

COMMENT ON COLUMN comment.id IS 'コメントのID';

COMMENT ON COLUMN comment.user_id IS '投稿したユーザーのID';

COMMENT ON COLUMN comment.article_id IS 'コメントを投稿した記事のID';

COMMENT ON COLUMN comment.empathy IS '共感';

COMMENT ON COLUMN comment.nice IS 'いいね';

COMMENT ON COLUMN comment.text IS '本文';

CREATE TABLE article(
    article_type USER - DEFINED NOT NULL,
    id SERIAL NOT NULL,
    user_id varchar(15) NOT NULL,
    empathy smallint NOT NULL DEFAULT 0,
    nice smallint NOT NULL DEFAULT 0,
    title varchar(30),
    text varchar(300) NOT NULL,
    PRIMARY KEY(id),
    CONSTRAINT article_user_id_fkey FOREIGN key(user_id) REFERENCES users(id)
);

COMMENT ON COLUMN article.article_type IS '記事のタイプ';

COMMENT ON COLUMN article.id IS '記事のID';

COMMENT ON COLUMN article.user_id IS '投稿したユーザーのID';

COMMENT ON COLUMN article.empathy IS '共感';

COMMENT ON COLUMN article.nice IS 'いいね';

COMMENT ON COLUMN article.title IS 'タイトル. article_type=consultatioinの時はNULL';

COMMENT ON COLUMN article.text IS '本文';