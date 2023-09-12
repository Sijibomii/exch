use Mix.Config


config :egusi, websocket_auth_timeout: 10_000
config :egusi, rabbit_url: "amqp://guest:guest@rabbitmq"


import_config "#{Mix.env()}.exs"
