// Parse mentions list once at load time to avoid split/map/filter overhead on every call.
const parsedMentions = (() => {
    const list = process.env.MENTIONS_HYDRATION || process.env.MENTIONS;
    if (list && list.trim().length > 0) {
        return list.split(',').map(id => id.trim()).filter(Boolean);
    }
    return [];
})();

function sanitizeErrorMessage(error) {
    const rawMessage = error?.stack || error?.message || String(error);
    return rawMessage.replace(/https:\/\/chat\.googleapis\.com\/[^\s)'"\]}]*/g, 'https://chat.googleapis.com/[REDACTED]');
}

function validateURL(urlStr) {
    if (!urlStr) {
        throw new Error('URL is empty or undefined');
    }
    let parsed;
    try {
        parsed = new URL(urlStr);
    } catch (error) {
        throw new Error('Invalid URL structure');
    }
    
    if (parsed.protocol !== 'https:') {
        throw new Error('Insecure URL scheme: only HTTPS is allowed');
    }
    if (parsed.hostname.toLowerCase() !== 'chat.googleapis.com') {
        throw new Error(`Unauthorized host '${parsed.hostname}': Only 'chat.googleapis.com' is allowed.`);
    }
    return parsed;
}

async function sendMessage(webhookURL, text) {
    let timeoutId;
    try {
        const validatedURL = validateURL(webhookURL);

        const controller = new AbortController();
        timeoutId = setTimeout(() => controller.abort(), 10000);

        const response = await fetch(validatedURL.toString(), {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ text }),
            signal: controller.signal
        });

        console.log(`Message sent to webhook, status:`, response.status);
    } catch (error) {
        const sanitizedMsg = sanitizeErrorMessage(error);
        console.error('Error sending message:', sanitizedMsg);
    } finally {
        if (timeoutId) {
            clearTimeout(timeoutId);
        }
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
