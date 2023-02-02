# Remote Media Pi

## Building

```shell
$ cargo build
```

## Testing

```shell
$ cargo test
```

## Docker

```shell 
$ docker build . -t rmp
$ docker run --rm -i rmp cargo build
$ docker run --rm -i rmp cargo test
```
