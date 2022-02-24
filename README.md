# telegram-sender

This projects sends a Telegram message to a given channel using a predefined bot token

## Installation

`sam build`

`sam local invoke -e events/sqs.json`

## Deployment

`sam build`

`sam deploy`

When pushed, Cloudformation updates every resource in AWS

## TODO list

- [ ] Local test needs to have aws-sdk installed only for local environment and not in the Lambda package

