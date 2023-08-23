defmodule Egusi do
  use Application

  def start(_type, _args) do
    import Supervisor.Spec, warn: false

    Egusi.Metric.PrometheusExporter.setup()
    Egusi.Metric.PipelineInstrumenter.setup()
    Egusi.Metric.UserSessions.setup()

    children = [
      # top-level supervisor for UserSession group

      Onion.Telemetry,
      Plug.Cowboy.child_spec(
        scheme: :http,
        plug: Ugwu,
        options: [
          port: String.to_integer(System.get_env("PORT") || "6000"),
          dispatch: dispatch(),
          protocol_options: [idle_timeout: :infinity]
        ]
      )
    ]

    opts = [strategy: :one_for_one, name: Egusi.Supervisor]

    # TODO: make these into tasks

    case Supervisor.start_link(children, opts) do
      {:ok, pid} ->

        {:ok, pid}

      error ->
        error
    end
  end


  defp dispatch do
    [
      {:_,
       [
         {"/socket", Ugwu.SocketHandler, []}
       ]}
    ]
  end

  #each token will have a running trading session that listed to rabbitmq for messages on that token. It then broadcasts it using pubsub to all subscribed users
  # each usersession should keep track of the running user trades, on every price update, the user subscribes to,  it calc the profit and loss and sends back.
  defp start_trading_sessions() do
    # get the list of all tokens and start a trading session
  end

  defp start_rabbits() do
    IO.puts("about to start_rabbits")

    # start the rabbit supervised
    # start the online rabbit supervised

    IO.puts("finished rabbits")
  end
end
