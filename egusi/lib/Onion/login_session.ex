defmodule Onion.LoginSession do
  use GenServer, restart: :temporary
  use AMQP

  defmodule Users do
    @type t :: %{
          user_id: String.t(),
          email: String.t(),
          wallets: [Wallet.t()],
          }

    defstruct user_id: nil,
              email: nil,
              wallets: []
  end

  defmodule State do
    @type t :: %__MODULE__{
            users: [Users.t()],
            chan: map()
          }
    defstruct users: [], nil
  end


  def start_supervised() do
    DynamicSupervisor.start_child(
      Onion.LoginDynamicSupervisor,
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

  def handle_cast({:send, msg}, %State{chan: chan, id: id} = state) do
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
      # map on the data here

    end

    # You might want to run payload consumption in separate Tasks in production
    # consume(chan, tag, redelivered, payload)
    {:noreply, state}
  end

  defp setup_queue(chan) do
    {:ok, _} = Queue.declare(chan, @receive_queue, durable: true)
    :ok = Queue.bind(chan, @receive_queue, @receive_exchange)
  end

end
