'use strict'

const TeleBot = require('telebot')

const AWS = require('aws-sdk')
AWS.config.update({region: 'us-east-1'})
const ssm = new AWS.SSM()

const sendMessage = async function (text, channelId, bot) {
    console.log('Sending telegram message: ' + text)
    await bot.sendMessage(channelId, text)
        .then(console.log('Message successfully sent'))
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
        const bot = new TeleBot(TELEGRAM_BOT_TOKEN)
        bot.start()
        await sendMessage(record.body, TELEGRAM_CHANNEL, bot)
        bot.stop()
    }
};
