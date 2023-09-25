defmodule Ugwu.Message.Request.Orderbook do
  use Ugwu.Message.Call,
  needs_auth: true
  @derive Jason.Encoder

  @primary_key false
  embedded_schema do
    field(:ticker_id, :integer)
    field(:orders, {:array, :map})
    field(:success, :boolean)
  end

  @impl true
  def changeset(initializer \\ %__MODULE__{}, data) do
    initializer
    |> cast(data, [:ticker_id])
    |> validate_required([:ticker_id])
  end


  defmodule Reply do
    use Ugwu.Message.Push

    @derive {Jason.Encoder, only: ~w(
      ticker_id
      success
      orders
    )a}

    schema "trades" do
      field(:ticker_id, :integer)
      field(:success, :boolean)
      field(:orders, {:array, :map})
    end


  end


  @impl true
  def execute(changeset!, state) do

    with {:ok, spec} <- apply_action(changeset!, :validation),
          # orders is a list
         {:ok, map} <-
          Egusi.Request.request_orderbook(
            state.trading_id,
            spec.ticker_id
          ) do
            IO.puts("ORDER REQUESTTTT")
          IO.inspect(map)
      {:reply, struct(__MODULE__,  map |> Map.put(:success, true)), state}
          else
            # error handling
            {:error, message } -> {:error, message}
    end
  end
end
