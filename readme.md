# Requirements

- Add `127.0.0.1   innershelter.org` to `etc/hosts` file
- Install (rust)[https://www.rust-lang.org/tools/install]
- Install (trunk)[https://trunkrs.dev/] for the client
- Install (cassandra)[https://formulae.brew.sh/formula/cassandra]
- Create the table
```
CREATE KEYSPACE IF NOT EXISTS inner_shelter WITH replication = {'class': 'SimpleStrategy', 'replication_factor': 1};

CREATE TABLE IF NOT EXISTS inner_shelter.users (
    username text PRIMARY KEY,
    password text
);
```

# Setup
- Run `cargo run -p server`
- Run `trunk serve --port 8081`
- Open `http://innershelter.org:8081/` from your browser
