# fakedata_server
> Usage example for [fakedata_generator](https://github.com/kevingimbel/fakedata_generator)

## Table of Contents
* [About](#about)
* [Usage](#usage)
  * [Rust](#rust)
  * [Docker](#docker)
  * [Accessing the API](#accessing-the-api)
* [Known issues](#known-issues) 
* [License](#license) 

## About
[⬆️ Back to Top](#table-of-contents)


## Usage
[⬆️ Back to Top](#table-of-contents)

### Rust

You can start the server with Rust by executing the following command.

```
$ cargo run
```

### Docker

There is a Docker image available at [https://hub.docker.com/r/kevingimbel/fakedata_server](https://hub.docker.com/r/kevingimbel/fakedata_server).

You can start the `fakedata_server` with the following command. 
```
$ docker run -it --rm -p 8000:8000 kevingimbel/fakedata_server
```

The container cannot be exited with `CTRL-C`, see [Known issues](#known-issues).


### Accessing the API

After starting the server in either of the two ways listed above, you can access [http://localhost:8000/](http://localhost:8000/). The `/` is required, see [Known issues](#known-issues).

## Known issues

* For some reason the index route binds to `localhost:8000/` with a `/` at the end but not to `localhost:8000` (no `/`).

* The Docker Container does not exit when pressing `CTRL-C`. When executed with `-it` you can press `CTRL-P` and `CTRL-Q` to detach from the container and then stop it with `docker stop`. 

## License
[⬆️ Back to Top](#table-of-contents)

[MIT License](https://github.com/kevingimbel/fakedata_server/blob/master/LICENSE) Copyright (c) 2019 Kevin Gimbel