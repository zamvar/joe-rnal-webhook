require('dotenv').config();
const cron = require('node-cron');
const { sendMessage } = require('./messenger');

const jobs = [
    {
        cronTime: '0 11 * * 1-5',
        // Every weekday at 11 AM
        webhookURL: process.env.WEBHOOK_URL_1,
        message: "Hi <users/all>, please don't forget to log in to Journal"
    },
    {
        cronTime: '0 18 * * 1-5',
        // Every weekday at 6 PM
        webhookURL: process.env.WEBHOOK_URL_1,
        message: "Hi <users/all>, please don't forget to log tasks in Journal"
    },
    {
        cronTime: '0 23 * * 1-5',
        // Every weekday at 11 PM
        webhookURL: process.env.WEBHOOK_URL_2,
        message: "Hi <users/all>, please don't forget to log tasks in Journal"
    }
];

for (const job of jobs) {
    if (!job.webhookURL) {
        console.warn(`[WARNING] Webhook URL is not defined for journal reminder at ${job.cronTime}. Messages will not be successfully delivered.`);
    }

    cron.schedule(job.cronTime, () => {
        sendMessage(job.webhookURL, job.message);
    }, {
        scheduled: true,
        timezone: 'Asia/Manila'
    });
    console.log(`Scheduled job to send "${job.message}" to ${job.webhookURL || 'undefined'} at ${job.cronTime} in Asia/Manila timezone`);
}

console.log('All Journal reminder jobs have been scheduled.');
