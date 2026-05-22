# Joe-rnal Webhook Reminders 🤖🥤

A multi-language reminder system providing automated Google Chat reminders using incoming webhooks. It features witty Tagalog/Taglish developer-focused hydration reminders and structured journal check-ins.

Both the **Node.js** and **Rust** implementations are included and share the **same configuration file** located in the root of the project.

---

## 📂 Project Structure

```
root/
├── nodejs/            # Node.js implementation & PM2 profiles
│   ├── scheduler.js   # Unified check-in & hydration scheduler
│   ├── hydrationReminder.js
│   ├── journalReminder.js
│   ├── messenger.js
│   └── ecosystem.config.js
├── rust/              # Rust implementation (high performance)
│   ├── src/           # Rust source files (main, scheduler, messenger, hydration, journal)
│   └── Cargo.toml
├── .env               # Shared environment configuration (untracked)
├── .example.env       # Shared environment configuration template
└── README.md
```

---

## 📝 Step 1: Environment Configuration (`.env`)

Create a shared `.env` file at the **root** of the project:

```bash
# In the root directory
cp .example.env .env
```

Open `.env` in your editor and configure the active webhook URLs and mentions list:
```env
# Google Chat Webhook URLs
WEBHOOK_URL_1=https://chat.googleapis.com/v1/spaces/...
WEBHOOK_URL_2=https://chat.googleapis.com/v1/spaces/...
WEBHOOK_URL_HYDRATION=https://chat.googleapis.com/v1/spaces/...

# Comma-separated list of 21-digit Google User IDs for Hydration Callouts
MENTIONS_HYDRATION=123456789012345678901,987654321098765432109
```

### 🕵️ How to Get Google Chat User IDs
1. Open [Google Chat](https://chat.google.com/) in your desktop browser.
2. **Right-click** on the person's profile name or avatar picture and select **Inspect**.
3. Look for the attribute: `data-member-id="users/123456789012345678901"` or `data-person-id="123456789012345678901"`.
4. Copy that **21-digit number** and add it to `MENTIONS_HYDRATION`.

---

## 🟢 Option A: Running the Node.js Implementation

The Node.js version is structured as clean daemons managed by PM2.

### 1. Installation
Because the workspace disk format limits symlink creation, dependencies must be installed without bin links:
```bash
cd nodejs
npm install --no-bin-links
```

### 2. Test Mentions Configuration
Verify that the loader parses and randomizes your root `.env` user list correctly:
```bash
cd nodejs
node -e "const path = require('path'); require('dotenv').config({ path: path.resolve('..', '.env') }); console.log('Parsed mention pattern:', require('./messenger').getHydrationMentions())"
```

### 3. Running with PM2
To run Node.js in the background:
```bash
cd nodejs

# Mode 1: Unified Mode (Recommended - single process to save memory)
pm2 start ecosystem.config.js --only JoeRnalUnified

# Mode 2: Split Mode (Separate processes for independent control)
pm2 start ecosystem.config.js --only JournalReminder,HydrationReminder
```

### 4. Monitoring PM2
```bash
# Check process list
pm2 status

# View live console output
pm2 logs JoeRnalUnified
```

---

## 🦀 Option B: Running the Rust Implementation

The Rust version is ultra-fast, memory-safe, and compiled into a single binary. It supports a built-in dry-run testing mode out-of-the-box.

### 1. Dry Run / Test (Recommended)
Verify configurations, formatted Tagalog messages, random mentions, and timezone calculation without sending active webhook requests:
```bash
# Directing cargo target to your home folder to bypass disk targets limitations
CARGO_TARGET_DIR=$HOME/.cargo/target-joe-rnal cargo run --manifest-path rust/Cargo.toml -- --mode unified --dry-run
```

### 2. Running in Production
```bash
CARGO_TARGET_DIR=$HOME/.cargo/target-joe-rnal cargo run --manifest-path rust/Cargo.toml -- --mode unified
```
*(You can also use `--mode journal` or `--mode hydration` to run split sub-tasks)*

### 3. Compiling a Release Binary
```bash
CARGO_TARGET_DIR=$HOME/.cargo/target-joe-rnal cargo build --release --manifest-path rust/Cargo.toml
```
Your optimized standalone binary will be built at: `$HOME/.cargo/target-joe-rnal/release/joe-rnal-webhook`
