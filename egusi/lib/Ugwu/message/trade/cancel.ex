defmodule Ugwu.Message.Trade.Cancel do
  use Ugwu.Message.Call,
  needs_auth: true

  @primary_key false
  embedded_schema do
    field(:ticker_id, :integer)
    field(:order_id, :integer)
  end

  @impl true
  def changeset(initializer \\ %__MODULE__{}, data) do
    initializer
    |> cast(data, [:ticker_id, :order_id])
    |> validate_required([:ticker_id, :order_id])
  end

  defmodule Reply do
    use Ugwu.Message.Push

    @derive {Jason.Encoder, only: ~w(
      ticker_id
      order_id
      success
    )a}

    schema "trades" do
      field(:ticker_id, :integer)
      field(:order_id, :integer)
      field(:success, :boolean)
    end
  end


  @impl true
  def execute(changeset!, state) do

    with {:ok, trade_spec} <- apply_action(changeset!, :validation),
         {:ok, %{trade: trade}} <-
          Egusi.Trade.Cancel.cancel(
            state.user.trading_id,
            trade_spec.ticker_id,
            trade_spec.order_id
          ) do
      {:reply, struct(__MODULE__,  Map.from_struct(trade) |> Map.update!(:success, fn _ -> true end)), state}
          else
            # error handling
            {:error, message } -> {:error, message}
    end
  end

end
