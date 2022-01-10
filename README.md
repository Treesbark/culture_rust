# culture_rust

A rust package that gets a random name from the Culture series' ships Minds.

## Getting started

Install Rust using the installation information at the Rust home page.

After cloning this project run

```console
$ cargo run
```

Note that the first time you run this command Cargo will build the API binaries.

### Testing

Once the API is running:

```console
$ curl http://localhost:8090/return-culture-ship-name
```