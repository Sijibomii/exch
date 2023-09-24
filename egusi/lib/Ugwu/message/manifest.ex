defmodule Ugwu.Message.Manifest do
  alias Ugwu.Message.Auth
  alias Ugwu.Message.Trade
  alias Ugwu.Message.Request

  @actions %{
    "auth:request" => Auth.Request,
    "trade:new" => Trade.New,
    "trade:modify" => Trade.Modify,
    "trade:cancel" => Trade.Cancel,
    "trade:listen" => Trade.Listen,
    "orders:all" => Request.Orderbook,
  }

  # verify that all of the actions are accounted for in the
  # operators list
  alias Ugwu.Message.Types.Operator
  require Operator

  @actions
  |> Map.values()
  |> Enum.each(fn module ->
    Operator.valid_value?(module) ||
      raise CompileError,
        description: "the module #{inspect(module)} is not a member of #{inspect(Operator)}"
  end)

  def actions, do: @actions
end
