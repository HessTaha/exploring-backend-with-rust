# Learning project

Simple api using rust axum mainly. It is a learning project to discover jwt authentification and backend development using rust.

## Setup install

**Required tools: Docker, docker-compose, rust, make**

- Set a .env file at the root of the project 

```shell
POSTGRES_PASSWORD=<TODO>
POSTGRES_USER=<TODO>
POSTGRES_DB=<TODO>
DATABASE_URL=<TODO>
```

- Start the db and the api server

```shell
make start-service
```

- Stop the service

```shell
make clean-service
```

Or restart

```shell
make restart-service
```

## Test the ready endpoint

```shell 
make get_readyz_response
```


