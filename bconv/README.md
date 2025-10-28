# Cli `bconv`

### Билдим тулзу
```
cd bconv
cargo build --release
```

### Проверяем
```
cd ..
./target/release/bconv --help
```

### Пользуемся
```
./target/release/bconv --input samples/data.csv --out-format=json
```