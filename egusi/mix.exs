defmodule Egusi.MixProject do
  use Mix.Project

  def project do
    [
      app: :egusi,
      version: "0.1.0",
      elixir: "~> 1.11",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      elixirc_paths: elixirc_paths(Mix.env),
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      mod: {Egusi, []},
      extra_applications: [:amqp, :logger, :crypto]
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:amqp, "~> 3.3"},
      {:plug_cowboy, "~> 2.6.1"},
      {:plug, "~> 1.0"},
      {:corsica, "~> 2.0"},
      {:phoenix_pubsub, "~> 2.1.3"},
      {:jose, "~> 1.11.6"},
      {:ecto_sql, "~> 3.10.2"},
      {:ecto_enum, "~> 1.4.0"},
      {:jason, "~> 1.4.1"},
      {:joken, "~> 2.6.0"},
      {:elixir_uuid, "~> 1.2.1"},
      {:net_address, "~> 0.3"},
      {:prometheus_ex, "~> 3.0.5"},
      {:prometheus_plugs, "~> 1.1.5"},
      {:timex, "~> 3.7.11"}
    ]
  end

  defp elixirc_paths(:test), do: ["lib", "test/support"]
  defp elixirc_paths(_), do: ["lib"]

end
