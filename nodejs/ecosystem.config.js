module.exports = {
  apps: [
    {
      name: 'JoeRnalUnified',
      script: 'scheduler.js',
      node_args: '--max-old-space-size=64',
      max_memory_restart: '80M',
      log_date_format: 'YYYY-MM-DD HH:mm:ss',
      error_file: 'logs/unified_err.log',
      out_file: 'logs/unified_out.log',
      log_type: 'json',
      max_size: '5M',
      retain: 2
    },
    {
      name: 'JournalReminder',
      script: 'journalReminder.js',
      node_args: '--max-old-space-size=64',
      max_memory_restart: '80M',
      log_date_format: 'YYYY-MM-DD HH:mm:ss',
      error_file: 'logs/journal_err.log',
      out_file: 'logs/journal_out.log',
      log_type: 'json',
      max_size: '5M',
      retain: 2
    },
    {
      name: 'HydrationReminder',
      script: 'hydrationReminder.js',
      node_args: '--max-old-space-size=64',
      max_memory_restart: '80M',
      log_date_format: 'YYYY-MM-DD HH:mm:ss',
      error_file: 'logs/hydration_err.log',
      out_file: 'logs/hydration_out.log',
      log_type: 'json',
      max_size: '5M',
      retain: 2
    },
    {
      name: 'JoeRnalRust',
      // By default points to the standard release target folder.
      // If you use custom CARGO_TARGET_DIR, update this to your absolute path (e.g. '/home/makoy/.cargo/target-joe-rnal/release/joe-rnal-webhook')
      script: '../rust/target/release/joe-rnal-webhook',
      args: '--mode unified',
      log_date_format: 'YYYY-MM-DD HH:mm:ss',
      error_file: 'logs/rust_err.log',
      out_file: 'logs/rust_out.log',
      log_type: 'json',
      max_size: '5M',
      retain: 2
    }
  ],
};
