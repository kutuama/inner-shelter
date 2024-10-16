Note that the client side code preferences are temporary, gonna replace it with godot or bevy at some point.

# Requirements

- Add `127.0.0.1   innershelter.org` to `etc/hosts` file
- Install [rust](https://www.rust-lang.org/tools/install)
- Install [trunk](https://trunkrs.dev/) for the client
- Install [cassandra](https://formulae.brew.sh/formula/cassandra)
- Run `cqlsh` and create the table
```
CREATE KEYSPACE IF NOT EXISTS inner_shelter WITH replication = {'class': 'SimpleStrategy', 'replication_factor': 1};

CREATE TABLE IF NOT EXISTS inner_shelter.users (
    username text PRIMARY KEY,
    password text
);
```

# Setup
- Run `cargo run -p service`
- Run `cargo run -p server`
- Run `npm run dev`
- Open `http://innershelter.org:8082/` from your browser

# Tools
- Get [repopack](https://github.com/yamadashy/repopack)
- Run `repopack --ignore "dist,public,target,Cargo.lock,readme.md,LICENSE"`
