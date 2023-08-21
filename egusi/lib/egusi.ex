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

  # each user can only be in one or more trading rooms. trading rooms hold the user ids of memebers
  # it calls their user sessions to send them some ws update
  defp start_trading_sessions() do

  end

  # TODO: bake this into the supervision tree itself by having the
  # supervisor fetch and look up the the list of online voice servers
  # by querying GCP tags.
  defp start_rabbits() do
    IO.puts("about to start_rabbits")

    # start the rabbit supervised
    # start the online rabbit supervised

    IO.puts("finished rabbits")
  end
end
