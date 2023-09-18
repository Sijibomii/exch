use Mix.Config


config :egusi, websocket_auth_timeout: 10_000
config :egusi, rabbit_url: "amqp://guest:guest@rabbits:5672/exch"


import_config "#{Mix.env()}.exs"
