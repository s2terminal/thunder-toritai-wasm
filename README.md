dev server start

```
$ docker-compose up --build
```

build

```
$ docker-compose exec app wasm-pack build --target web --out-name wasm --out-dir ./static
```
