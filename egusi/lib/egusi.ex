defmodule Egusi do
  use Application

  def start(_type, _args) do
    import Supervisor.Spec, warn: false

    # Egusi.Metric.PrometheusExporter.setup()
    # Egusi.Metric.PipelineInstrumenter.setup()
    # Egusi.Metric.UserSessions.setup()

    children = [
      Onion.Supervisors.UserSession,
      Onion.Supervisors.TickerSession,
      Onion.Supervisors.LoginSession,
      Onion.Supervisors.BalanceRabbit,
      Onion.Supervisors.ClientRabbit,
      Onion.Supervisors.MDRabbit,
      Onion.Supervisors.OrderRabbit,
      # Onion.Telemetry,
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

  defp start_trading_sessions() do
    # get the list of all tokens and start a trading session
    # read from file and start all tickers
    file_path = "/tokens/ticker.txt"
    case File.read(file_path) do
      {:ok, content} ->
        # Successfully read the file
        data = Jason.decode!(payload)
        Enum.each(data, &start_session/1)

      {:error, reason} ->
        # Failed to read the file
        IO.puts("Error reading the file: #{reason}")
    end
  end
  alias Onion.TickerSession
  defp staert_session(data) do
    TickerSession.start_supervised(
      ticker_id: data["id"]
    )
  end

  defp start_rabbits() do
    # start rabbits with ids 0, 1
    IO.puts("about to start_rabbits")

    # start the rabbit supervised
    # start the online rabbit supervised

    IO.puts("finished rabbits")
  end
end
