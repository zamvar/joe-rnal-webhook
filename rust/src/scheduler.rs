use chrono::Utc;
use chrono_tz::Asia::Manila;
use cron::Schedule;
use std::env;
use std::str::FromStr;
use std::time::Duration;
use tokio::time::sleep;
use rand::seq::SliceRandom;
use reqwest::Client;
use serde_json::json;

pub const HYDRATION_MESSAGES: &[&str] = &[
    "💧 **Hydration Check!** Uminom ka muna ng tubig saglit. Wag puro trabaho/code!",
    "🥤 **Tubig-tubig din pag may time!** Tumayo ka muna riyan at mag-refill ng tumbler mo.",
    "🌊 **Huy, inom muna ng tubig!** Baka tuyong-tuyo na lalamunan mo riyan. Refresh muna!",
    "🍃 **Quick water break naman diyan!** Pang-refresh lang ng utak mong pagod na.",
    "💧 **Refill-refill din ng tubig pag may time.** Stay hydrated para iwas sakit ng ulo!",
    "🥛 **Water break muna!** Kahit tatlong lagok lang, sapat na para magising ang diwa mo.",
    "💧 **Inom na ng tubig lods!** Baka dehydrated ka na riyan nang hindi mo napapansin.",
    "🥤 **Refill station alert!** Pagkakataon mo na para maglakad-lakad at kumuha ng malamig na tubig.",
    "🌊 **Huy!** Kape ka nang kape, mag-tubig ka rin pag may time!",
    "🍃 **Hydrate bago ma-burnout.** Isang basong tubig muna diyan bago tapusin ang kasunod na task!",
    "💧 **Time check: Oras na para mag-water break.** Stretch nang kaunti sabay tagay ng tubig!",
    "🥤 **Tubig > Kape at Milk Tea.** Opo, masakit ang katotohanan pero kailangan mong magtubig ngayon.",
    "💧 **Lagok-lagok din pag may time.** Yung code mo may error na, baka utak mo lang ang dehydrated.",
    "🥛 **Ang uminom ng tubig, gumaganda/gwumagwapo.** Sige na, uminom ka na para gumana na rin ang code mo.",
    "💧 **Hydration Level Check!** Di ba 70% ng mundo ay tubig? Make sure ganun din ang lagay ng katawan mo ngayon.",
    "🥤 **Isang basong tubig naman diyan.** Pwede mong takasan ang production bugs pero hindi ang dehydration.",
    "🥛 **Tubig is key.** Baka naman kaya ka nai-stuck sa bug kasi kulang ka lang sa H2O. Subukan mo uminom bago mag-debug.",
    "🍃 **Tumayo, mag-stretch, magtubig.** 5-minute break lang para sa kidney mong nagmamakaawa sa trabaho.",
    "💧 **Refill tumbler checkpoint!** Ito na ang mahiwagang sign para maglakad papuntang water dispenser.",
    "🥤 **Wag matigas ang ulo, uminom ng tubig ngayon!** Para kang dry season kung hindi ka iinom.",
    "🌊 **H2O update available!** Version 1.0 is ready for deployment. Install now sa pamamagitan ng pag-lagok ng tubig.",
    "💧 **Water.exe is not responding.** Kelangan mo nang mag-input ng isang basong tubig para mag-reload ang system mo.",
    "🥤 **Huy dev!** Baka tuyo na ang keyboard mo pero mas tuyo ang lalamunan mo. Tayo na at mag-water break!",
    "🥛 **Bawal ang dry run sa lalamunan.** Uminom ka muna ng tubig para swabe ang takbo ng workflow mo.",
    "🌊 **Refill Refill Din Pag May Time!** Baka mas marami pa ang lines of code mo kaysa sa milliliter ng nainom mong tubig ngayon.",
    "🍃 **Commit muna bago uminom.** I-save ang changes sa code, tapos i-save ang kidney sa pamamagitan ng pagtubig.",
    "💧 **Hydrate and Conquer.** Mas madaling mag-solve ng logic errors kung hindi dehydrated ang control unit (utak) mo.",
    "🥤 **Huy, break muna sa screen!** Kumuha ng tubig at tumitig sa malayo para makapag-relax din ang mata mo.",
    "🥛 **Drink water now.** Parang database query lang yan, kelangan mo ng connection (refill) para gumana.",
    "🌊 **Status Code 200: Water level is good.** Kung hindi pa, alam mo na ang dapat mong gawin ngayon.",
    "🍃 **Tubig is life, code is just logic.** Wag kalimutan ang pinaka-importanteng resource ng katawan mo.",
    "💧 **Uminom ng tubig para sa clear skin at clear logic.** Win-win situation, di ba? Inom na!",
    "🥤 **Gusto mo ba ng productivity hack?** Ang tawag dun ay PAG-INOM NG TUBIG. Yes, scientifically proven tapos libre pa!"
];

#[derive(clap::ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum RunMode {
    Unified,
    Journal,
    Hydration,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub webhook_url_1: Option<String>,
    pub webhook_url_2: Option<String>,
    pub webhook_url_hydration: Option<String>,
    pub mentions_hydration: Vec<String>,
}

impl Config {
    pub fn load() -> Self {
        // Attempt to load .env file
        let _ = dotenvy::dotenv();

        let webhook_url_1 = env::var("WEBHOOK_URL_1").ok().filter(|s| !s.trim().is_empty());
        let webhook_url_2 = env::var("WEBHOOK_URL_2").ok().filter(|s| !s.trim().is_empty());
        
        let webhook_url_hydration = env::var("WEBHOOK_URL_HYDRATION")
            .ok()
            .filter(|s| !s.trim().is_empty())
            .or_else(|| webhook_url_1.clone());

        let mentions_str = env::var("MENTIONS_HYDRATION")
            .ok()
            .or_else(|| env::var("MENTIONS").ok())
            .unwrap_or_default();

        let mentions_hydration: Vec<String> = mentions_str
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        Config {
            webhook_url_1,
            webhook_url_2,
            webhook_url_hydration,
            mentions_hydration,
        }
    }
}

pub enum MessageTemplate {
    Static(String),
    Hydration,
}

pub struct Job {
    pub name: String,
    pub cron_time: String,
    pub webhook_url: Option<String>,
    pub message_template: MessageTemplate,
}

pub fn get_jobs(mode: RunMode, config: &Config) -> Vec<Job> {
    let mut jobs = Vec::new();

    let has_journal = match mode {
        RunMode::Unified | RunMode::Journal => true,
        RunMode::Hydration => false,
    };

    let has_hydration = match mode {
        RunMode::Unified | RunMode::Hydration => true,
        RunMode::Journal => false,
    };

    if has_journal {
        jobs.push(Job {
            name: "Journal Reminder (11 AM)".to_string(),
            cron_time: "0 0 11 * * Mon-Fri *".to_string(),
            webhook_url: config.webhook_url_1.clone(),
            message_template: MessageTemplate::Static("Hi <users/all>, please don't forget to log in to Journal".to_string()),
        });
        jobs.push(Job {
            name: "Journal Reminder (6 PM)".to_string(),
            cron_time: "0 0 18 * * Mon-Fri *".to_string(),
            webhook_url: config.webhook_url_1.clone(),
            message_template: MessageTemplate::Static("Hi <users/all>, please don't forget to log tasks in Journal".to_string()),
        });
        jobs.push(Job {
            name: "Journal Reminder (11 PM)".to_string(),
            cron_time: "0 0 23 * * Mon-Fri *".to_string(),
            webhook_url: config.webhook_url_2.clone(),
            message_template: MessageTemplate::Static("Hi <users/all>, please don't forget to log tasks in Journal".to_string()),
        });
    }

    if has_hydration {
        jobs.push(Job {
            name: "Hydration Reminder (9 AM - 5 PM Hourly)".to_string(),
            cron_time: "0 0 9-17 * * Mon-Fri *".to_string(),
            webhook_url: config.webhook_url_hydration.clone(),
            message_template: MessageTemplate::Hydration,
        });
    }

    jobs
}

pub fn get_hydration_message() -> &'static str {
    let mut rng = rand::thread_rng();
    HYDRATION_MESSAGES.choose(&mut rng).copied().unwrap_or(HYDRATION_MESSAGES[0])
}

pub fn get_hydration_mentions(mentions: &[String]) -> String {
    if !mentions.is_empty() {
        let mut rng = rand::thread_rng();
        if let Some(random_user) = mentions.choose(&mut rng) {
            return format!("<users/all> (lalo na kay <users/{}>)", random_user);
        }
    }
    "<users/all>".to_string()
}

pub async fn send_message(client: &Client, webhook_url: &str, text: &str) -> Result<(), reqwest::Error> {
    let payload = json!({ "text": text });
    let response = client.post(webhook_url)
        .json(&payload)
        .send()
        .await?;
    println!("Message sent to webhook, status: {}", response.status());
    Ok(())
}

pub async fn start_scheduler(mode: RunMode, dry_run: bool) -> Result<(), Box<dyn std::error::Error>> {
    println!("Initializing Joe-rnal webhook scheduler (Rust edition)...");

    let config = Config::load();

    // Validate webhook URLs at startup
    if config.webhook_url_1.is_none() {
        println!("⚠️ WARNING: Environment variable WEBHOOK_URL_1 is not defined. Messages for this webhook will fail to send.");
    } else {
        println!("✅ WEBHOOK_URL_1 is configured.");
    }
    if config.webhook_url_2.is_none() {
        println!("⚠️ WARNING: Environment variable WEBHOOK_URL_2 is not defined. Messages for this webhook will fail to send.");
    } else {
        println!("✅ WEBHOOK_URL_2 is configured.");
    }
    if config.webhook_url_hydration.is_none() {
        println!("⚠️ WARNING: Environment variable WEBHOOK_URL_HYDRATION is not defined. Messages for this webhook will fail to send.");
    } else {
        println!("✅ WEBHOOK_URL_HYDRATION is configured.");
    }

    let jobs = get_jobs(mode, &config);

    if dry_run {
        println!("\n==========================================");
        println!("🧪 DRY-RUN TEST MODE ACTIVE");
        println!("==========================================");
        println!("Configured mode: {:?}", mode);
        println!("Mentions configured: {:?}", config.mentions_hydration);
        
        for job in &jobs {
            println!("\n📋 Job: {}", job.name);
            println!("   Cron time: {}", job.cron_time);
            println!("   Webhook configured: {}", job.webhook_url.is_some());
            
            // Format example message
            let sample_msg = match &job.message_template {
                MessageTemplate::Static(msg) => msg.clone(),
                MessageTemplate::Hydration => {
                    let mentions = get_hydration_mentions(&config.mentions_hydration);
                    let body = get_hydration_message();
                    format!("{}\n{}", mentions, body)
                }
            };
            println!("   Sample formatted output:\n------------------------------------------\n{}\n------------------------------------------", sample_msg);

            // Compute next 5 occurrences
            match Schedule::from_str(&job.cron_time) {
                Ok(schedule) => {
                    println!("   Next 5 occurrences in Asia/Manila timezone:");
                    let mut upcoming = schedule.upcoming(Manila);
                    for idx in 1..=5 {
                        if let Some(next) = upcoming.next() {
                            println!("      {}. {}", idx, next);
                        } else {
                            break;
                        }
                    }
                }
                Err(e) => {
                    println!("   ⚠️ Invalid Cron pattern: {}", e);
                }
            }
        }
        println!("\nDry run completed successfully. No HTTP calls were sent.");
        return Ok(());
    }

    // Run active jobs in spawned Tokio tasks
    let client = Client::new();
    let mut handles = Vec::new();

    for job in jobs {
        if job.webhook_url.is_none() {
            println!("⚠️ Skipping job \"{}\" scheduling due to missing webhook URL.", job.name);
            continue;
        }

        let webhook_url = job.webhook_url.unwrap();
        let client_clone = client.clone();
        let mentions_clone = config.mentions_hydration.clone();
        
        let schedule = match Schedule::from_str(&job.cron_time) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to parse cron for job \"{}\": {}", job.name, e);
                continue;
            }
        };

        println!("📅 Scheduled job \"{}\" to run at \"{}\" (timezone: Asia/Manila)", job.name, job.cron_time);

        let handle = tokio::spawn(async move {
            let mut last_run = Utc::now().with_timezone(&Manila);
            loop {
                // Determine next execution time
                let mut upcoming = schedule.after(&last_run);
                let next_run = match upcoming.next() {
                    Some(time) => time,
                    None => {
                        eprintln!("No more scheduled runs for job \"{}\"", job.name);
                        break;
                    }
                };

                let now = Utc::now().with_timezone(&Manila);
                if next_run > now {
                    let duration = match (next_run - now).to_std() {
                        Ok(d) => d,
                        Err(_) => Duration::from_secs(0),
                    };
                    sleep(duration).await;
                }

                // Construct message
                let message = match &job.message_template {
                    MessageTemplate::Static(msg) => msg.clone(),
                    MessageTemplate::Hydration => {
                        let mentions = get_hydration_mentions(&mentions_clone);
                        let body = get_hydration_message();
                        format!("{}\n{}", mentions, body)
                    }
                };

                println!("🔔 Triggering job \"{}\" at {}", job.name, Utc::now().with_timezone(&Manila));
                if let Err(e) = send_message(&client_clone, &webhook_url, &message).await {
                    eprintln!("❌ Error executing job \"{}\": {}", job.name, e);
                }

                // Update last run time to prevent repeat execution
                last_run = next_run;
            }
        });

        handles.push(handle);
    }

    if handles.is_empty() {
        println!("⚠️ No jobs are scheduled. Exiting.");
        return Ok(());
    }

    println!("🚀 All reminders have been successfully scheduled in a single Tokio event loop!");

    // Wait indefinitely for all tasks to run
    for handle in handles {
        let _ = handle.await;
    }

    Ok(())
}
