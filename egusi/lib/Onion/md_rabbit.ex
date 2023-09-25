defmodule Onion.MDRabbit do
  use GenServer
  use AMQP

  defmodule State do
    @type t :: %{
            id: String.t(),
            chan: map()
          }

    defstruct id: "", chan: nil
  end

  def start_supervised(id) do
    DynamicSupervisor.start_child(
      Onion.MDRabbitClientDynamicSupervisor,
      {__MODULE__, id}
    )
  end

  def start_link(id) do
    GenServer.start_link(
      __MODULE__,
      id,
      name: via(id)
    )
  end

  defp via(id), do: {:via, Registry, {Onion.MDRabbitClientRegistry, id}}

  @receive_exchange "exch"
  @receive_queue "incremental"

  def init(id) do
    IO.puts("md rabbits comming up")
    {:ok, conn} =
      Connection.open(Application.get_env(:egusi, :rabbit_url, "amqp://guest:guest@rabbits:5672/exch"))

    {:ok, chan} = Channel.open(conn)
    setup_queue(id, chan)

    queue_to_consume_1 = @receive_queue

    # Register the GenServer process as a consumer
    {:ok, _consumer_tag} = Basic.consume(chan, queue_to_consume_1, nil, no_ack: true)
    IO.puts("md rabbits done comming up")
    {:ok, %State{chan: chan, id: id}}
  end

  def send(id, msg) do
    GenServer.cast(via(id), {:send, msg})
  end

  def handle_cast({:send, _msg}, %State{chan: _chan} = state) do
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
    # how to make sure that each ticker is up to date
    case data do
      %{"op" => "MARKET-UPDATE-CLEAR"} -> :ok

      # order filled, rejected,..

      # publish order types depending on ticker_id
      %{"op" => _} ->
        IO.puts("md rabbits: received a new message")
        IO.inspect(data)
        # change it to operatio here
        Onion.TickerSession.add_order(data["data"]["ticker_id"], data)
        :ok
    end

    # You might want to run payload consumption in separate Tasks in production
    # consume(chan, tag, redelivered, payload)
    {:noreply, state}
  end

  defp setup_queue(_id, chan) do
    {:ok, _} = Queue.declare(chan, @receive_queue, durable: false)
    :ok = Queue.bind(chan, @receive_queue, @receive_exchange)
  end
end
