module.exports = {
  apps: [
    {
      name: 'JoeRnalRust',
      // By default points to the standard release target folder inside rust/
      // If you compile using custom CARGO_TARGET_DIR, update this to your absolute path (e.g. '/home/makoy/.cargo/target-joe-rnal/release/joe-rnal-webhook')
      script: './target/release/joe-rnal-webhook',
      args: '--mode unified',
      log_date_format: 'YYYY-MM-DD HH:mm:ss',
      error_file: 'logs/rust_err.log',
      out_file: 'logs/rust_out.log',
      log_type: 'json',
      max_size: '5M',
      retain: 2
    }
  ]
};
