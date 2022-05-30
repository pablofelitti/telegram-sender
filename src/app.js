'use strict'

const AWS = require('aws-sdk')
AWS.config.update({region: 'us-east-1'})
const ssm = new AWS.SSM()
const axios = require('axios')

const sendMessage = async function (text, channelId, tokenId) {
    console.log('Sending telegram message: ' + text)

    let data = JSON.stringify({
        "chat_id": channelId,
        "text": text
    })

    let config = {
        method: 'post',
        url: 'https://api.telegram.org/bot' + tokenId + '/sendMessage',
        headers: {
            'Content-Type': 'application/json'
        },
        data: data
    }

    await axios(config)
        .then(function (response) {
            console.log(JSON.stringify(response.data))
        })
        .catch(function (error) {
            console.log(error)
        })
}

const globalTelegramParameterStore = '/telegram-ids/';

async function getParameters(environmentId) {
    const query = {Path: globalTelegramParameterStore + environmentId}
    let ssmResponse = await ssm.getParametersByPath(query).promise();
    return ssmResponse.Parameters
}

function getParameter(parameters, environmentId, key) {
    return parameters.filter(it => it.Name === globalTelegramParameterStore + environmentId + '/' + key)[0].Value;
}

exports.lambdaHandler = async (event, context) => {

    for (let record of event.Records) {

        const environmentId = record.messageAttributes["EnvironmentId"].stringValue;
        const channel = record.messageAttributes["Channel"].stringValue;

        let parameters = await getParameters(environmentId)
        const TELEGRAM_BOT_TOKEN = getParameter(parameters, environmentId, 'token')
        const TELEGRAM_CHANNEL = getParameter(parameters, environmentId, channel)
        await sendMessage(record.body, TELEGRAM_CHANNEL, TELEGRAM_BOT_TOKEN)
    }
};
