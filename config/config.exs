import Config

# Logging configuration
config :logger, :default_formatter,
  format: "[$level] $dateT$time $message $metadata\n",
  metadata: [:application]
