# Joe-rnal Webhook Reminders 🤖🥤

A lightweight, reliable, and secure Node.js application that schedules automated Google Chat reminders using incoming webhooks. It features witty Tagalog/Taglish developer-focused hydration reminders and structured journal check-ins.

Managed via **PM2** as two independent, isolated daemons.

---

## Features

1. **Hydration Reminder (`hydration-reminder`)**
   * Runs **hourly** (9:00 AM - 5:00 PM) on weekdays (Monday - Friday).
   * Cycles through **40+ casual, witty Tagalog/Taglish messages** to keep the team entertained and hydrated.
   * Tags the whole channel (`<users/all>`) and randomly calls out one specific developer from a configured list on each run.
2. **Journal Reminder (`journal-reminder`)**
   * Runs at **11:00 AM, 6:00 PM, and 11:00 PM** every day.
   * Always tags `<users/all>` to remind everyone to submit their journal entries.

---

## 🛠️ Step 1: Installation & Setup

Because this project is hosted on a workspace disk format that restricts symlink creation, dependencies must be installed without bin links:

```bash
# Install dependencies safely
npm install --no-bin-links
```

---

## 📝 Step 2: Environment Configuration (`.env`)

Create a file named `.env` in the root of the project:

```bash
touch .env
```

Open `.env` in your editor and add the following template:

```env
# Google Chat Webhook URLs
WEBHOOK_URL_1=https://chat.googleapis.com/v1/spaces/...
WEBHOOK_URL_HYDRATION=https://chat.googleapis.com/v1/spaces/...

# Comma-separated list of 21-digit Google User IDs for Hydration Callouts
MENTIONS_HYDRATION=123456789012345678901,987654321098765432109,111222333444555666777
```

---

## 🕵️ How to Get Google Chat User IDs

Incoming Webhooks cannot mention users by name or email. You must provide their unique **21-digit Google Profile ID**.

### The "Inspect Element" Method (Fastest)
1. Open [Google Chat](https://chat.google.com/) in your desktop browser.
2. Go to the space where the person is located.
3. **Right-click** on the person's profile name or avatar picture and select **Inspect** to open Developer Tools.
4. Locate the HTML element. Look for the attribute:
   * `data-member-id="users/123456789012345678901"`
   * OR `data-person-id="123456789012345678901"`
5. Copy the 21-digit number and paste it into the `MENTIONS_HYDRATION` list in your `.env` file, separated by commas.

---

## 🧪 Step 3: Test Your Mentions Configuration

Before launching, you can run a dry-test command to verify that the script successfully reads and randomizes your `.env` user list:

```bash
node -e "require('dotenv').config(); console.log('Parsed mention pattern:', require('./messenger').getHydrationMentions())"
```

*Expected Output:*
`Parsed mention pattern: <users/all> (lalo na kay <users/123456789012345678901>)` *(with a random ID from your list)*

---

## 🚀 Step 4: Running with PM2

The application supports two modes of execution depending on your preference. **Choose only one option below** to prevent duplicate reminders:

### Option A: Unified Mode (Recommended for low RAM)
Runs both the journal and hydration schedulers inside a **single** Node.js process. This reduces memory overhead by ~50% (~35MB total).

```bash
# Start in Unified Mode
pm2 start ecosystem.config.js --only JoeRnalUnified
```

### Option B: Split Mode (Recommended for independent control)
Runs the reminders in **two separate** processes. This allows you to stop or pause one reminder without affecting the other.

```bash
# Start in Split Mode
pm2 start ecosystem.config.js --only JournalReminder,HydrationReminder
```

---

## 📊 Managing & Monitoring PM2

### Check Running Status
```bash
pm2 status
```

### View Live Logs
To see logs in real-time (helps verify cron execution or dry runs):
```bash
# View logs for all running PM2 processes
pm2 logs

# View logs for specific processes
pm2 logs JoeRnalUnified
pm2 logs HydrationReminder
pm2 logs JournalReminder
```

### Pause / Stop a Process
If you are in **Split Mode** and want to temporarily pause hydration reminders (e.g., during a holiday weekend):
```bash
pm2 stop HydrationReminder
```

### Restart a Process
```bash
pm2 restart JoeRnalUnified
```
