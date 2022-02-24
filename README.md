# telegram-sender

This projects sends a Telegram message to a given channel using a predefined bot token

## Installation

Just `npm install` and run any package.json task

## Tasks

| Name                           | Description                                             |
|--------------------------------|---------------------------------------------------------|
| `npm run local`                | run locally service to process the web                  |
| `npm run start`                | run scheduled process (Dockerfile uses this)            |
| `npm run test`                 | run unit tests                                          |

## Deployment

When pushed, AWS Cloudformation updates every resource

## TODO list

- [ ] Local test needs to have aws-sdk installed only for local environment and not in the Lambda package

