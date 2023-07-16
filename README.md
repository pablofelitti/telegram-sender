# telegram-sender

This projects sends a Telegram message to a given channel using a predefined bot token

## To run locally using cargo lambda

```
cd rust_app
cargo lambda watch
```

and in another terminal run:
```
cd rust_app
cargo lambda invoke --data-file ../events/sqs.json
```

## To run locally using AWS SAM

```
sam build
sam local invoke -e events/sqs.json
```

## Deployment

`sam build`

`sam deploy`

When pushed, Cloudformation updates every resource in AWS

## TODO list

- [ ] Local test needs to have aws-sdk installed only for local environment and not in the Lambda package

