<div align="center">
    <h1>qujila</h1>
</div>

# wokrking draft for **qujila** DB library

## How to Use
1. Define schema in `src/schema.rs` using `qujila::schema!` macro. This will be editor-completable to some extent (by idea of **wrapping macro_rules**).

```rust
/* src/schema.rs */

qujila::schema! {
    User {
        id:       SERIAL where PRIMARY_KEY,
        name:     VARCHAR(20) where NOT_NULL,
        password: VARCHAR(20) where NOT_NULL,
    },

    Task {
        id:          SERIAL where PRIMARY_KEY,
        user_id:     REFERENCING(User::id),
        title:       VARCHAR(20) where NOT_NULL,
        description: TEXT,
    },
}
```

<br/>

2. Execute migration by `qujila sync` at the top of the project. `qujila` command will be installable by `cargo install qujila-cli`.

```sh
$ qujila sync ${DB_URL}
```

<br/>

3. `qujila::schema!` will automatically generate ORM codes. Use them in the project.

```rust
/* src/sample_1.rs */

use qujila::Connection;
use crate::schema::{User, newUser};

async fn signup(
    c: &Connection,
    name: String,
    password: String,
) -> Result<User, crate::MyError> {
    let n_existing = c.Count::<User>()
        .WHERE(|u| [
            u.name.eq(&name),
            u.passwod.eq(&password),
        ])
        .await?;

    if n_existing != 0 {
        return Err(MyError::ExistingUser)
    }

    let new_user = c.Create(
        newUser{
            name,
            password,
        }
    ).await?;

    Ok(new_user)
}
```

```rust
/* src/sample_2.rs */

use qujila::Connection;
use crate::schema::Task;

async fn get_tasks(
    conn: &Connection,
    user_id: usize,
) -> Result<Vec<Task>, MyError> {
    conn.All::<Task>()
        .WHERE(|task|
            task.user_id.eq(&user_id)
        )
        .ORDER_ASC(|task| task.user_id)
        .ORDER_ASC(|task| task.title)
        .await.map_err(|e| e.into())
}
```

<br/>
<br/>

## Available Operations
- `qujila::Connection::`
  - `Create( NEW_TABLE_ENTITY { /* */ })`
  - `_Create( NEW_TABLE_ENTITY { /* */ })`
    - `await` ( calling `IntoFuture` )
- `qujila::Connection::`
  - `First::<TABLE_ENTITY>()`
  - `All::<TABLE_ENTITY>()`
  - `Update::<TABLE_ENTITY>()`
  - `_Update::<TABLE_ENTITY>()`
  - `Delete::<TABLE_ENTITY>()`
  - `_Delete::<TABLE_ENTITY>()`
  - `Count::<TABLE_ENTITY>()`
    - `/* conditions */`
      - `await` ( calling `IntoFuture` )

<br/>

Methods of name `_Method` are **not-returning-entity** verion of `Method` .
