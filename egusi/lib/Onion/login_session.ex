defmodule Onion.LoginSession do
  use GenServer, restart: :temporary
  use AMQP

  defmodule Wallet do
    @type t :: %{
            id: String.t(),
            balance: number()
          }

    defstruct id: nil,
              balance: nil
  end

  defmodule User do
    @type t :: %{
          user_id: String.t(),
          trading_client_id: Integer.t(),
          last_order_number: Integer.t(),
          last_seq_num: Integer.t(),
          email: String.t(),
          wallet: Wallet.t(),
          }

    defstruct user_id: nil,
              email: nil,
              wallet: nil,
              trading_client_id: nil,
              last_order_number: nil,
              last_seq_num: nil

  end

  defmodule State do
    @type t :: %__MODULE__{
            users: [User.t()],
            chan: map()
          }
    defstruct users: [], chan: nil
  end


  def start_supervised() do
    DynamicSupervisor.start_child(
      Onion.LoginSessionDynamicSupervisor,
      {__MODULE__}
    )
  end

  def start_link() do
    GenServer.start_link(
      __MODULE__,
      name: via()
    )
  end

  defp via(), do: {:via, Registry, {Onion.LoginSessionRegistry, 1}}


  @receive_exchange "exch"
  @receive_queue "authentication"
  def init() do
    {:ok, conn} =
      Connection.open(Application.get_env(:egusi, :rabbit_url, "amqp://guest:guest@localhost"))

    {:ok, chan} = Channel.open(conn)
    setup_queue(chan)

    queue_to_consume = @receive_queue
    IO.puts("queue_to_consume: " <> queue_to_consume)
    # Register the GenServer process as a consumer
    {:ok, _consumer_tag} = Basic.consume(chan, queue_to_consume, nil, no_ack: true)
    {:ok, %State{chan: chan, users: []}}
  end

  def send(id, msg) do
    GenServer.cast(via(), {:send, msg})
  end

  def call(_id, msg) do
    GenServer.call(via(), msg)
  end

  defp user_info_impl(_reply, user_id, state) do
    user = Enum.find(state.users, fn user -> user.user_id == user_id end)
    {:reply, user, state}
  end

  # defp add_user(data, )

  def handle_call({:get_user_info, user_id}, reply, state), do: user_info_impl(reply, user_id, state)

  def handle_cast({:send, msg}, %State{chan: chan} = state) do
    AMQP.Basic.publish(chan, "", @recieve_queue, Jason.encode!(msg))
    {:noreply, state}
  end

  def handle_info({:basic_consume_ok, %{consumer_tag: _consumer_tag}}, state) do
    {:noreply, state}
  end

  # Sent by the broker when the consumer is unexpectedly cancelled (such as after a queue deletion)
  def handle_info({:basic_cancel, %{consumer_tag: _consumer_tag}}, state) do
    {:stop, :normal, state}
  end

  # Confirmation sent by the broker to the consumer process after a Basic.cancel
  def handle_info({:basic_cancel_ok, %{consumer_tag: _consumer_tag}}, state) do
    {:noreply, state}
  end

  def handle_info(
        {:basic_deliver, payload, %{delivery_tag: _tag, redelivered: _redelivered}},
        %State{} = state
      ) do
    data = Jason.decode!(payload)

    case data do
      %{"op" => "USER-LOGIN"} ->
        {:noreply, %{state | users: [ %User{

          user_id: data["data"]["user_id"],
          email: data["data"]["email"],
          trading_client_id: data["data"]["trading_client_id"],
          last_order_number: data["data"]["last_order_number"],
          last_seq_num: data["data"]["last_seq_num"],
          wallet: %Wallet{
            id: data["data"]["wallet"]["id"],
            balance: data["data"]["wallet"]["balance"],
          }
        } | state.users ]}}

        %{"op" => "USER-LOGIN-NO-WALLET"} -> {:noreply, %{state | users: [ %User{
          user_id: data["data"]["user_id"],
          email: data["data"]["email"],
          trading_client_id: data["data"]["trading_client_id"],
          last_order_number: data["data"]["last_order_number"],
          last_seq_num: data["data"]["last_seq_num"],
          wallet: nil
        } | state.users ]}}

      _ -> {:noreply, state}
    end
  end

  defp setup_queue(chan) do
    {:ok, _} = Queue.declare(chan, @receive_queue, durable: true)
    :ok = Queue.bind(chan, @receive_queue, @receive_exchange)
  end

end
