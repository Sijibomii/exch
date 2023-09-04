defmodule Ugwu.Message.Trade.New do
  use Ugwu.Message.Call,
  needs_auth: true

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
  def execute(changeset, state) do
    with {:ok, request} <- apply_action(changeset, :validate),
         {:ok, user} <- Egusi.Trade.create(request) do

      {:reply, user, %{state | user: user }}
    else
      # don't tolerate malformed requests with any response besides closing
      # out websocket.
      _ -> {:close, 4001, "invalid_authentication"}
    end
  end

end
