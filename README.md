# vvote
vvote is an evoting system build with actix-web(modify [jelly](https://github.com/secretkeysio/jelly-actix-web-starter) template) and tera template\
The voting check/mechanisim is base on voter ip-address and web-browser(fingerprintjs)

# Applications features
- voting candidate management
- manage available office/position for candidate
- manage election year/session
- export voting report to excel sheet

# Installation instruction

## setup environment variable
$ cp .env.example .env

## Important settings
- JELLY_DOMAIN
- DATABASE_URL
- REDIS_CONNECTION_STRING

# install sqlx-cli
```
$ cargo install sqlx-cli --no-default-features --features native-tls,postgres

$ sqlx migrate run
```

# Build and run application
```
## if using local server for image upload
$ cargo run
## if using cloudinary for image upload
$ cargo run --features cloudinary
```

# Usage
use the **JELLY_DOMAIN** url set to vote (http://your_domain.com)\
use **JELLY_DOMAIN/admin** to login to admin dashbord (http://your_domain.com/admin)\
login email is `admin@gmail.com`\
login passpord is `secret`

# Online Demo
[Voting url](https://vvote.onrender.com)\
[Admin url](https://vvote.onrender.com/admin)

# License
vvote is open-sourced software licensed under the [MIT license](https://opensource.org/licenses/MIT).