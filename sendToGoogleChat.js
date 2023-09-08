
require('dotenv').config();
const axios = require('axios');
const cron = require('node-cron');
const tzOffset = require('tz-offset');

const jobs = [
    {
        cronTime: '0 11 * * 1-5',
        // # Every weekday at 11 AM
        webhookURL: process.env.WEBHOOK_URL_1,
        message: 'Hi <users/all>, Please dont forget to login to Journal'
    },
    {
        cronTime: '0 18 * * 1-5',
        //  # Every weekday at 6 PM
        webhookURL: process.env.WEBHOOK_URL_1,
        message: 'Hi <users/all>, Please dont forget to log tasks in journal'
    },
    {
        cronTime: '0 23 * * 1-5',
        //  # Every weekday at 11 PM
        webhookURL: process.env.WEBHOOK_URL_2,
        message: 'Hi <users/all>, Please dont forget to log tasks in journal'
    },
    {
        cronTime: '* * * * *',
        //  # Every Minute
        webhookURL: process.env.WEBHOOK_URL_3,
        message: 'Hi <users/all>, This is a test please disregard'
    }
];

async function sendMessage(webhookURL, text) {
    const data = {
        'text': text
    };

    try {
        const response = await axios.post(webhookURL, data);
        console.log('Message sent:', response.status);
    } catch (error) {
        console.error('Error sending message:', error);
    }
}

for (let job of jobs) {
    cron.schedule(job.cronTime, () => {
        sendMessage(job.webhookURL, job.message);
    }, {
        scheduled: true,
        timezone: "Asia/Manila"
    });
    console.log(`Scheduled job to send "${job.message}" to ${job.webhookURL} at ${job.cronTime} in Asia/Manila timezone`);
}

console.log("All jobs have been scheduled.");
