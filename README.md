# telegram-sender

This projects is a RUST AWS Lambda that sends a Telegram message to a given telegram group based on parameters

## To run locally using cargo lambda

```bash
cd rust_app
cargo lambda watch
```

and in another terminal run:
```bash
cd rust_app
cargo lambda invoke --data-file ../events/sqs.json
```

## To run locally using AWS SAM

```
sam build
sam local invoke -e events/sqs.json
```

> **Warning**
> Neither way to run locally (cargo or SAM) seem to work in local environment at the moment

## Deployment

```bash
sam build
sam deploy
```

## TODO list

- [ ] Continue refactoring RUST code in general, it works but it isn't nice :)
- [ ] Find a reliable way to test locally

