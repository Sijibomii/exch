defmodule Ugwu.Message.Trade.New do
  use Ugwu.Message.Call,
  needs_auth: true
  @derive Jason.Encoder
  @primary_key false
  embedded_schema do
    field(:ticker_id, :integer)
    field(:side, :string)
    field(:price, :integer)
    field(:qty, :integer)
  end

  @impl true
  def changeset(initializer \\ %__MODULE__{}, data) do
    initializer
    |> cast(data, [:ticker_id, :side, :price, :qty])
    |> validate_required([:ticker_id, :side, :price, :qty])
  end

  defmodule Reply do
    use Ugwu.Message.Push

    @derive {Jason.Encoder, only: ~w(
      ticker_id
      side
      price
      qty
      success
    )a}

    schema "trades" do
      field(:ticker_id, :integer)
      field(:side, :string)
      field(:price, :integer)
      field(:qty, :integer)
      field(:success, :boolean)
    end
  end


  @impl true
  def execute(changeset!, state) do

    with {:ok, trade_spec} <- apply_action(changeset!, :validation),
         {:ok, %{trade: trade}} <-
          Egusi.Trade.Create.create(
            state.trading_id,
            trade_spec.ticker_id,
            trade_spec.side,
            trade_spec.price,
            trade_spec.qty
          ) do
      {:reply, struct(__MODULE__,  trade |> Map.put(:success, true)), state}
          else
            # error handling
            {:error, message } -> {:error, message}
    end
  end

end
