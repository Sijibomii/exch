defmodule Ugwu.Message.Request.Orderbook do
  use Ugwu.Message.Call,
  needs_auth: true

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

    with {:ok, spec} <- apply_action(changeset!, :validation),
         {:ok, %{orders: orders}} <-
          Egusi.Request.request_orderbook(
            state.user.trading_id,
            spec.ticker_id
          ) do
      {:reply, struct(__MODULE__,  Map.from_struct(orders) |> Map.update!(:success, fn _ -> true end)), state}
          else
            # error handling
            {:error, message } -> {:error, message}
    end
  end
end
