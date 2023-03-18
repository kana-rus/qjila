<div align="center">
    <h1>qjila</h1>
</div>

# wokrking draft for **qjila** DB library

## How to Use
1. Define schema using `qjila::schema` macro. This will be editor-completable to some extent (by idea of **wrapping macro_rules**).

```rust
/* src/schema.rs */

qjila::schema! {
    User {
        id: SERIAL as usize where PRIMARY_KEY,
        name:     VARCHAR(20) where NOT_NULL,
        password: VARCHAR(20) where NOT_NULL,
    },

    Task {
        id: SERIAL as usize where PRIMARY_KEY,
        user_id:     REFERENCING(User.id),
        title:       VARCHAR(20) where NOT_NULL,
        description: TEXT,
    },
}
```

<br/>

2. Execute migration by `qjila migrate` at the top of the project. `qjila` command will be installable by `cargo install qjila-cli`.

```sh
$ qjila migrate
```

<br/>

3. `qjila::schema!` will automatically generate ORM codes. Use them in the project.

```rust
/* src/sample_1.rs */

use qjila::Connection;
use crate::schema::{new, User};

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
        new::User{
            name,
            password,
        }
    ).await?;

    Ok(new_user)
}
```

```rust
/* src/sample_2.rs */

use qjila::Connection;
use crate::schema::Task;

async fn get_tasks(
    conn: &Connection,
    user_id: usize,
) -> Result<Vec<Task>, MyError> {
    conn.All()
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
- `qjila::Connection::`
  - `Create( NEW_TABLE_ENTITY { /* */ })`
  - `_Create( NEW_TABLE_ENTITY { /* */ })`
    - `await` ( calling `IntoFuture` )
- `qjila::Connection::`
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

`NEW_TABLE_ENTITY` is made by `schema::new::/* TABLE_ENTITY_NAME */` .\
Methods of name `_Method` are **not-returning-entity** verion of `Method` .
