defmodule Ugwu.Message.Trade.Listen do
  use Ugwu.Message.Call,
  needs_auth: true

  @derive Jason.Encoder
  @primary_key false
  embedded_schema do
    field(:ticker_id, :integer)
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
    )a}

    schema "trades" do
      field(:ticker_id, :integer)
      field(:success, :boolean)
    end
  end


  @impl true
  def execute(changeset!, state) do

    with {:ok, trade_spec} <- apply_action(changeset!, :validation),
         {:ok, %{trade: trade}} <-
          Egusi.Trade.Listen.listen(
            state.trading_id,
            trade_spec.ticker_id
          ) do
      {:reply, struct(__MODULE__,  trade |> Map.put(:success, true)), state}
          else
            # error handling
            {:error, message } -> {:error, message}
    end
  end

end
