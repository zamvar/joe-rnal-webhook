module.exports = {
  apps: [{
    name: 'Joe',
    script: 'sendToGoogleChat.js',
    max_memory_restart: '300M',
    log_date_format: 'YYYY-MM-DD HH:mm:ss',
    error_file: 'logs/err.log',
    out_file: 'logs/out.log',
    log_type: 'json',
    max_size: '5M', // 10MB
    retain: 2 // keep 20 log files.
  }],
};
