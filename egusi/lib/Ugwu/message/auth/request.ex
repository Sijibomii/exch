defmodule Ugwu.Message.Auth.Request do
  use Ugwu.Message.Call,
    needs_auth: false

  @primary_key false
  embedded_schema do
    field(:accessToken, :string)
    field(:refreshToken, :string)
  end


  @impl true
  def changeset(initializer \\ %__MODULE__{}, data) do
    initializer
    |> cast(data, [:accessToken, :refreshToken])
    |> validate_required([:accessToken])
  end

  defmodule Reply do
    use Ugwu.Message.Push

    @derive {Jason.Encoder, only: ~w(
      id
      email
    )a}

    @primary_key {:id, :binary_id, []}
    schema "users" do
      field(:email, :string)
    end
  end

  @impl true
  def execute(changeset, state) do
    with {:ok, request} <- apply_action(changeset, :validate),
         {:ok, user} <- Egusi.Auth.authenticate(request, state.ip) do

      {:reply, user, %{state | user: user }}
    else
      # don't tolerate malformed requests with any response besides closing
      # out websocket.
      _ -> {:close, 4001, "invalid_authentication"}
    end
  end
end
