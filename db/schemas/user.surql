DEFINE TABLE user SCHEMAFULL
    PERMISSIONS
        FOR select FULL
        FOR update WHERE id = $auth.id
        FOR create, delete NONE;

DEFINE FIELD username ON user TYPE string;
DEFINE FIELD email ON user TYPE string ASSERT string::is::email($value);
DEFINE FIELD pwd ON user TYPE string;
DEFINE FIELD registered_at ON user TYPE datetime DEFAULT time::now();

DEFINE INDEX unique_email ON user COLUMNS email UNIQUE;

DEFINE SCOPE user_scope SESSION 30d
    SIGNUP (
        CREATE user
        SET
            username = $username,
            email = $email,
            pwd = crypto::argon2::generate($pwd)
    )
    SIGNIN (
        SELECT *
        FROM user
        WHERE username = $username AND crypto::argon2::compare(pwd, $pwd)
    );
