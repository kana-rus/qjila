<div align="center">
    <h1>qjila</h1>
</div>

# wokrking draft for **qjila** DB library

1. Define schema using `qjila::schema` macro. This will be editor-completable to some extent (by idea of **wrapping macro_rules**).

```rust
/* src/schema.rs */

qjila::schema! {
    // table entity
    User {
        id: SERIAL as usize where PRIMARY_KEY,
        name:     VARCHAR(20) where NOT_NULL,
        password: VARCHAR(20) where NOT_NULL,
    },

    // table entity
    Task {
        id: SERIAL as usize where PRIMARY_KEY,
        user_id:     REFERENCING(User.id),
        title:       VARCHAR(20) where NOT_NULL,
        description: TEXT,
    },

    // sub entity
    type UserData = {
        name: User.name,
        password: User.password,
    }
}
```

2. Execute migration by `qjila migrate` at the top of the project. `qjila` command will be installable by `cargo install qjila-cli`.

```sh
$ qjila migrate
```

3. `qjila::schema!` will automatically generates ORM codes. Use them in the project.

```rust
/* src/sample.rs */

use qjila::Connection;
use crate::schema::{User, newUser};

async fn signup(
    conn: &Connection,
    name: String,
    password: String,
) -> Result<User, crate::MyError> {
    let n_existing = User::Count(&conn)
        .WHERE(|u| [
            u.name.eq(&name),
            u.password.eq(&password),
        ])
        .await?;

    if n_existing != 0 {
        return Err(MyError::ExistingUser)
    }

    let new_user = User::Create(&conn)
        .ONE(newUser{
            name,
            password,
        })
        .await?;
    
    Ok(new_user)
}
```
