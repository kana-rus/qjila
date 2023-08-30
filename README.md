<div align="center">
    <h1>qujila</h1>
</div>

# Working Draft for **qujila** DB library ( any codes here don't run now )

## Example; How to Use
1. Define tbales with `qujila::table` attribute and `qujila::column`.

`src/my_db_schema.rs`
```rust
qujila::schema! {
    mod User {
        let id          = usize.auto_increment();
        let name        = String;
        let password    = String;
        let profile     = String;
        let created_at  = DateTime.default_now();
        let updated_at  = DateTime. /* ... */;
    }

    mod Task {
        let id          = usize.auto_increment();
        let user_id     = usize.references::<User, "id">();
        let title       = String;
        let description = String;
        let created_at  = DateTime.default_now();
        let updated_at  = DateTime. /* ... */;
    }
}
```

<br/>

2. Execute migration by `qujila sync` at the top of the project. `qujila` command will be installable by `cargo install qujila-cli`.

```sh
$ qujila sync my_db_schema ${DB_URL}
```
Then, you can put `--emit-sql` flag to emit `up.sql` and `down.sql` into your migration directory：
```sh
$ qujila sync my_db_schema ${DB_URL} --emit-sql ${migration_directory_path}
```

<br/>

3. `qujila::schema!` will automatically generate ORM codes. Use them in the project.

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

## TODOs
- **top priority**: support **relations**
- **second priority**: parameterize conditions
- support **SELECT**
- support **JOIN**
- support **TRANSACTION**

<br/>

## License
`qujila` is licensed under MIT LICENSE ([LICENSE-MIT](https://github.com/kana-rus/qujila/blob/main/LICENSE-MIT) or [https://opensource.org/licenses/MIT](https://opensource.org/licenses/MIT)).
