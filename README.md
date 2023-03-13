# API Exercise

A web framework built upon Axum using the Rust language.
Makes requests to [JSON Placeholder API](https://jsonplaceholder.typicode.com/).

# Table of Contents

- [API Exercise](#api-exercise)
- [Table of Contents](#table-of-contents)
  - [Installation](#installation)
  - [Running the Server](#running-the-server)
    - [Running Tests](#running-tests)
  - [Docker](#docker)
  - [Generating documentation](#generating-documentation)
  - [Endpoints](#endpoints)
    - [Healthcheck](#healthcheck)
      - [Response](#response)
    - [GET Posts](#get-posts)
      - [Response](#response-1)
    - [POST Posts](#post-posts)
      - [Request](#request)
      - [Response](#response-2)
  - [License](#license)

## Installation

Create an .env file at the root of your project:

```shell
touch .env
```

Now add environment values for local development:

```ini
RUST_BACKTRACE=0
RUST_LOG="api_exercise=debug,axum=info"
SERVER=127.0.0.1:8000
```

## Running the Server

To startup the server:

```shell
cargo run
```

### Running Tests

To run all of the tests:

```shell
cargo test
```

## Docker

To build a Docker image of the application:

```shell
docker build -t api_exercise .
```

Once the image is built, you can run the container in port 3000:

```shell
docker run -it --rm --env-file=.env.docker -p 3000:3000 --name api_exercise api_exercise
```

## Generating documentation

```shell
cargo doc --no-deps --open
```

## Endpoints

### Healthcheck

Determine if the system is healthy.

`GET /health`

#### Response

`200 OK`

Example:

```shell
curl -X GET http://127.0.0.1:3000/health
```

### GET Posts

Wrapper around GET posts endpoint from JSON placeholder api

`GET /posts/{id}`

#### Response

`200 OK`

```json
{
	"id": 1,
	"title": "sunt aut facere repellat provident occaecati excepturi optio reprehenderit",
	"body": "quia et suscipit\nsuscipit recusandae consequuntur expedita et cum\nreprehenderit molestiae ut ut quas totam\nnostrum rerum est autem sunt rem eveniet architecto",
	"userId": 1
}
```

Example:

```shell
curl -X GET http://127.0.0.1:8000/posts/1
```

### POST Posts

Wrapper around POST posts endpoint from JSON placeholder api

`POST /posts`

#### Request

| Param  | Type    | Description                     | Required | Validations           |
| ------ | ------- | ------------------------------- | :------: | --------------------- |
| title  | String  | The post's title                |   yes    | at least 2 characters |
| body   | String  | The post's body                 |   yes    | none                  |
| userId | Integer | The user creating the post's id |   yes    | none                  |

#### Response

`200 OK`

```json
{
	"id": 101
}
```

Example:

```shell
curl -X POST http://127.0.0.1:8000/posts
```

## License

This project is licensed under:

- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)
