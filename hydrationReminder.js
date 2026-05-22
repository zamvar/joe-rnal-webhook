require('dotenv').config();
const cron = require('node-cron');
const { sendMessage, getHydrationMentions } = require('./messenger');

const jobs = [
    {
        cronTime: '0 9-17 * * 1-5',
        // Every hour from 9 AM to 5 PM on weekdays
        webhookURL: process.env.WEBHOOK_URL_HYDRATION || process.env.WEBHOOK_URL_1,
        message: () => {
            const hydrationMessages = [
                '💧 **Hydration Check!** Uminom ka muna ng tubig saglit. Wag puro trabaho/code!',
                '🥤 **Tubig-tubig din pag may time!** Tumayo ka muna riyan at mag-refill ng tumbler mo.',
                '🌊 **Huy, inom muna ng tubig!** Baka tuyong-tuyo na lalamunan mo riyan. Refresh muna!',
                '🍃 **Quick water break naman diyan!** Pang-refresh lang ng utak mong pagod na.',
                '💧 **Refill-refill din ng tubig pag may time.** Stay hydrated para iwas sakit ng ulo!',
                '🥛 **Water break muna!** Kahit tatlong lagok lang, sapat na para magising ang diwa mo.',
                '💧 **Inom na ng tubig lods!** Baka dehydrated ka na riyan nang hindi mo napapansin.',
                '🥤 **Refill station alert!** Pagkakataon mo na para maglakad-lakad at kumuha ng malamig na tubig.',
                '🌊 **Huy!** Kape ka nang kape, mag-tubig ka rin pag may time!',
                '🍃 **Hydrate bago ma-burnout.** Isang basong tubig muna diyan bago tapusin ang kasunod na task!',
                '💧 **Time check: Oras na para mag-water break.** Stretch nang kaunti sabay tagay ng tubig!',
                '🥤 **Tubig > Kape at Milk Tea.** Opo, masakit ang katotohanan pero kailangan mong magtubig ngayon.',
                '💧 **Lagok-lagok din pag may time.** Yung code mo may error na, baka utak mo lang ang dehydrated.',
                '🥛 **Ang uminom ng tubig, gumaganda/gwumagwapo.** Sige na, uminom ka na para gumana na rin ang code mo.',
                '💧 **Hydration Level Check!** Di ba 70% ng mundo ay tubig? Make sure ganun din ang lagay ng katawan mo ngayon.',
                '🥤 **Isang basong tubig naman diyan.** Pwede mong takasan ang production bugs pero hindi ang dehydration.',
                '🥛 **Tubig is key.** Baka naman kaya ka nai-stuck sa bug kasi kulang ka lang sa H2O. Subukan mo uminom bago mag-debug.',
                '🍃 **Tumayo, mag-stretch, magtubig.** 5-minute break lang para sa kidney mong nagmamakaawa sa trabaho.',
                '💧 **Refill tumbler checkpoint!** Ito na ang mahiwagang sign para maglakad papuntang water dispenser.',
                '🥤 **Wag matigas ang ulo, uminom ng tubig ngayon!** Para kang dry season kung hindi ka iinom.',
                '🌊 **H2O update available!** Version 1.0 is ready for deployment. Install now sa pamamagitan ng pag-lagok ng tubig.',
                '💧 **Water.exe is not responding.** Kelangan mo nang mag-input ng isang basong tubig para mag-reload ang system mo.',
                '🥤 **Huy dev!** Baka tuyo na ang keyboard mo pero mas tuyo ang lalamunan mo. Tayo na at mag-water break!',
                '🥛 **Bawal ang dry run sa lalamunan.** Uminom ka muna ng tubig para swabe ang takbo ng workflow mo.',
                '🌊 **Refill Refill Din Pag May Time!** Baka mas marami pa ang lines of code mo kaysa sa milliliter ng nainom mong tubig ngayon.',
                '🍃 **Commit muna bago uminom.** I-save ang changes sa code, tapos i-save ang kidney sa pamamagitan ng pagtubig.',
                '💧 **Hydrate and Conquer.** Mas madaling mag-solve ng logic errors kung hindi dehydrated ang control unit (utak) mo.',
                '🥤 **Huy, break muna sa screen!** Kumuha ng tubig at tumitig sa malayo para makapag-relax din ang mata mo.',
                '🥛 **Drink water now.** Parang database query lang yan, kelangan mo ng connection (refill) para gumana.',
                '🌊 **Status Code 200: Water level is good.** Kung hindi pa, alam mo na ang dapat mong gawin ngayon.',
                '🍃 **Tubig is life, code is just logic.** Wag kalimutan ang pinaka-importanteng resource ng katawan mo.',
                '💧 **Uminom ng tubig para sa clear skin at clear logic.** Win-win situation, di ba? Inom na!',
                '🥤 **Gusto mo ba ng productivity hack?** Ang tawag dun ay PAG-INOM NG TUBIG. Yes, scientifically proven tapos libre pa!'
            ];
            const randomIndex = Math.floor(Math.random() * hydrationMessages.length);
            return hydrationMessages[randomIndex];
        }
    }
];

for (const job of jobs) {
    if (!job.webhookURL) {
        console.warn(`[WARNING] Webhook URL is not defined for hydration reminder at ${job.cronTime}. Messages will not be successfully delivered.`);
    }

    cron.schedule(job.cronTime, () => {
        const text = typeof job.message === 'function' ? job.message() : job.message;
        const mentions = getHydrationMentions();
        sendMessage(job.webhookURL, `${mentions}\n${text}`);
    }, {
        scheduled: true,
        timezone: 'Asia/Manila'
    });
    const logMessage = typeof job.message === 'function' ? '[Dynamic Hydration Reminder]' : job.message;
    console.log(`Scheduled job to send "${logMessage}" to ${job.webhookURL || 'undefined'} at ${job.cronTime} in Asia/Manila timezone`);
}

console.log('All Hydration reminder jobs have been scheduled.');
