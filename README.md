<div align="center">
    <h1>qujila</h1>
</div>

# Working Draft for **qujila** DB library ( any codes here don't run now )

## Example; How to Use

0. Add to `dependencies` and install cli :

```sh
$ cargo add qujila
$ cargo install qujila-cli
```

<br/>

1. Define your DB schema in `.primsa` file :

`schema.prisma`
```prisma
generator client {
    provider = "qujila"
    output   = "src/my_db_schema"
}

datasource db {
    provider = "postgres"
    url      = env("DATABASE_URL")
}

# `cargo add chrono` to use DateTime

model User {
    id         Int      @id @default(autoincrement())
    name       String
    password   String
    profile    String?
    created_at DateTime @default(now())
    posts      Post[]
}

model Post {
    id         Int      @id @default(autoincrement())
    title      String
    content    String
    published  Boolean  @default(false)
    auther     User     @relation(fields: [auther_id], references: [id])
    auther_id  Int
    created_at DateTime @default(now())
}
```

<br/>

2. Generate client code and execute migration :

```sh
$ qujila sync
```

You can emit migration history as SQL files by `--emit-sql` flag :

```sh
$ qujila sync --emit-sql my_migrations_dir
```

In production, execute these migration files :

```sh
$ qujila migrate my_migrations_dir
# This executes all .sql files (in expected format)
# including ones you manually added
```

<br/>

3. Use client code in your project.

Here Mr.Sample uses `ohkami` on `tokio`：

`src/main.rs`
```rust
mod handler;
mod schema;

use ohkami::prelude::*;
use crate::handler::{
    users::*,
};

#[tokio::main]
async fn main() {
    qujila::connect("DB_URL")
        .max_connections(1024)
        .await?;

    Ohkami::new((
        "api/users"
            .POST(create_user),
        "api/users/:id"
            .GET(get_user)
            .PATCH(update_user)
            .DELETE(delete_user),
    )).howl(":3000").await
}
```

`src/handler/users.rs`
```rust
use ohkami::{prelude::*, utils::Payload};
use crate::my_db_schema::{User};

#[Payload(JSON)]
#[derive(serde::Deserialize)]
struct CreateUserRequest {
    name:     String,
    password: String,
    profile:  String,
}

async fn create_user(c: Context, payload: CreateUserRequest) -> Response {
    let CreateUserRequest { name, password, profile } = payload;

    if User(|u|
        u.name.eq(&name) &
        u.password.eq(hash_func(&password))
    ).exists().await? {
        return c.InternalServerError().text("user already exists")
    }

    let new_user = User::created()
        .name(name)
        .password(hash_func(&password))
        .profile(profile)
        .await?;
    c.Created(new_user)
}

async fn get_user(c: Context, id: usize) -> Response {
    let user = User(|u| u.id.eq(id)).single().await?;
    c.OK().json(user)
}

#[Payload(JSON)]
#[derive(serde::Deserialize)]
struct UpdateUserRequest {
    name:     Option<String>,
    password: Option<String>,
    profile:  Option<String>,
}

async fn update_user(c: Context
    (id,): (usize,),
    payload: UpdateUserRequest,
) -> Response {
    let target = User(|u| u.id.eq(id));

    if target.count().await? != 1 {
        return c.InternalServerError().text("user is not single")
    }

    let updater = target.update();
    if let Some(new_name) = &payload.name {
        updater.set_name(new_name)
    }
    if let Some(new_password) = &payload.password {
        updater.set_password(hash_func(new_password))
    }
    if let Some(new_profile) = &payload.profile {
        updater.set_profile(new_profile)
    }
    updater.await?;
    c.NoContent()
}

async fn delete_user(c: Context, id: usize) -> Response {
    let target = User(|u| u.id.eq(id));
    
    if target.count().await? != 1 {
        return c.InternalServerError().text("user is not single")
    }

    target.delete().await?;
    c.NoContent()
}
```

<br/>

## License
`qujila` is licensed under MIT LICENSE ([LICENSE-MIT](https://github.com/kana-rus/qujila/blob/main/LICENSE-MIT) or [https://opensource.org/licenses/MIT](https://opensource.org/licenses/MIT)).
