### Run backend

```sh
cargo watch -x "run" --ignore 'uploads/*' --ignore 'frontend/*'
```

### Run frontend

```sh
pnpm install
pnpm run dev -- --open --host
```


### Run migrations

Install CLI :

```sh
sudo apt-get install libpq-dev
cargo install diesel_cli
```

Commands :
```sh
diesel migration run
diesel migration redo
```