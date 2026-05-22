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
    }
  ],
};
