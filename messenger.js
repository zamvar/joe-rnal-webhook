// Parse mentions list once at load time to avoid split/map/filter overhead on every call.
const parsedMentions = (() => {
    const list = process.env.MENTIONS_HYDRATION || process.env.MENTIONS;
    if (list && list.trim().length > 0) {
        return list.split(',').map(id => id.trim()).filter(Boolean);
    }
    return [];
})();

async function sendMessage(webhookURL, text) {
    if (!webhookURL) {
        console.error('Validation Error: Webhook URL is missing or undefined.');
        return;
    }

    try {
        const response = await fetch(webhookURL, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ text })
        });
        console.log(`Message sent to webhook, status:`, response.status);
    } catch (error) {
        console.error('Error sending message:', error);
    }
}

function getHydrationMentions() {
    if (parsedMentions.length > 0) {
        const randomUser = parsedMentions[Math.floor(Math.random() * parsedMentions.length)];
        return `<users/all> (lalo na kay <users/${randomUser}>)`;
    }
    return '<users/all>';
}

module.exports = { sendMessage, getHydrationMentions };
