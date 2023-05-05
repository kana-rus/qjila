<div align="center">
    <h1>qujila</h1>
</div>

# Working Draft for **qujila** DB library ( any codes here don't run now )

## Example; How to Use
1. Define schema in `src/schema.rs` using `qujila::schema!` macro. This will be editor-completable to some extent (by idea of **wrapping macro_rules**).

`src/schema.rs`
```rust
qujila::schema! {
    User {
        id:         __ID__,
        name:       VARCHAR(20) where NOT_NULL,
        password:   VARCHAR(20) where NOT_NULL,
        profile:    TEXT,
        created_at: __CREATED_AT__,
        updated_at: __UPDATED_AT__,
    }
}
```

<br/>

2. Execute migration by `qujila sync` at the top of the project. `qujila` command will be installable by `cargo install qujila-cli`.

```sh
$ qujila sync ${DB_URL}
```
Then, you can put `--emit-sql` flag to emit `up.sql` and `down.sql` into your migration directory：
```sh
$ qujila sync ${DB_URL} --emit-sql ${migration_directory_path}
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
async fn main() -> Result<()> {
    qujila::spawn("DB_URL")
        .max_connections(1024)
        .await?;

    Ohkami::new([
        "api/users"
            .POST(create_user),
        "api/users/:id"
            .GET(get_user)
            .PATCH(update_user)
            .DELETE(delete_user),
    ]).howl(":3000").await
}
```

`src/handler/users.rs`
```rust
use ohkami::{
    prelude::*,
    request::RequestBody,
};
use crate::schema::User;


#[RequestBody(JSON)]
struct CreateUserRequest {
    name:     String,
    password: String,
    profile:  String,
}

async fn create_user(c: Context,
    payload: CreateUserRequest
) -> Response<User> {
    let CreateUserRequest {
        name, password, profile
    } = payload;

    if User(|u|
        u.name.eq(&name) &
        u.password.eq(hash_func(&password))
    ).exists().await? {
        c.InternalServerError("user already exists")
    } else {
        let new_user = User::Create{
            name,
            password: hash_func(&password),
            profile,
        }.await?;
        c.Created(new_user)
    }
}

async fn get_user(c: Context, id: usize) -> Response<User> {
    let user = User(|u| u.id.eq(id)).Single().await?;
    c.json(user)
}

#[RequestBody(JSON)]
struct UpdateUserRequest {
    name:     Option<String>,
    password: Option<String>,
    profile:  Option<String>,
}

async fn update_user(c: Context
    (id,): (usize,),
    payload: UpdateUserRequest,
) -> Response<()> {
    let target = User(|u| u.id.eq(id));

    if target.is_single() {
        let updater = target.update();
        if let Some(new_name) = payload.name {
            updater.set_name(new_name)
        }
        if let Some(new_password) = payload.password {
            updater.set_password(hash_func(new_password))
        }
        if let Some(new_profile) = payload.profile {
            updater.set_profile(new_profile)
        }
        updater.await?;
        c.NoContent()
    } else {
        c.InternalServerError("user not single")
    }
}

async fn delete_user(c: Context, id: usize) -> Response<()> {
    let target = User(|u| u.id.eq(id));
    
    if target.is_single().await? {
        target.delete().await?;
        c.NoContent()
    } else {
        c.InternalServerError("user not single")
    }
}
```

<br/>

## TODOs
- **top priority**: support **relations**
- **second priority**: parameterize conditions
- support **JOIN**
- support **TRANSACTION**

<br/>

## License
`qujila` is licensed under MIT LICENSE ([LICENSE-MIT](https://github.com/kana-rus/qujila/blob/main/LICENSE-MIT) or [https://opensource.org/licenses/MIT](https://opensource.org/licenses/MIT)).
