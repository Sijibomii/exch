defmodule Onion.SnapshotRabbit do
  use GenServer
  use AMQP

  # this will act as a dispatcher for order rabbit. ticker session will collect orders from usersession and send to order_rabbit who will dispatch to rabbit
  defmodule State do
    @type t :: %{
            id: String.t(),
            chan: map()
          }

    defstruct id: "", chan: nil
  end

  def start_supervised(id) do
    DynamicSupervisor.start_child(
      Onion.SnapshotRabbitClientDynamicSupervisor,
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

  defp via(id), do: {:via, Registry, {Onion.SnapshotRabbitClientRegistry, id}}

  # @send_queue "order"
  @receive_exchange "exch"
  @receive_queue "snapshot"
  # @receive_queue_2 "incremental"

  def init(id) do
    {:ok, conn} =
      Connection.open(Application.get_env(:egusi, :rabbit_url, "amqp://guest:guest@localhost"))

    {:ok, chan} = Channel.open(conn)
    setup_queue(id, chan)

    queue_to_consume_1 = @receive_queue
    IO.puts("queue_to_consume: " <> queue_to_consume)
    # Register the GenServer process as a consumer
    {:ok, _consumer_tag} = Basic.consume(chan, queue_to_consume_1, nil, no_ack: true)
    {:ok, %State{chan: chan, id: id}}
  end

  def send(id, msg) do
    GenServer.cast(via(id), {:send, msg})
  end

  def handle_cast({:send, msg}, %State{chan: chan, id: id} = state) do
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
      %{"op" => "MARKET-UPDATE-CLEAR"} -> # look at the ticker id in the message and notify the appropraite tickersession to get ready

      # publish order types depending on ticker_id
      %{"op" => "MARKET-UPDATE-ADD"} ->

        # each ticker session must keep a snapshot messages queue
        # check if first snapshot message is clear
        # check snapshot sync every 5 sec?
        #
      %{"op" => "MARKET-UPDATE-MODIFY"} ->

      %{"op" => "MARKET-UPDATE-CANCEL"} ->

      %{"op" => "MARKET-UPDATE-TRADE"} ->

      %{"op" => "MARKET-UPDATE-SNAPSHOT-START"} ->  # clear snapshot seq_num stored by rabbit and start from 0

      %{"op" => "MARKET-UPDATE-SNAPSHOT-END"} ->
    end

    # You might want to run payload consumption in separate Tasks in production
    # consume(chan, tag, redelivered, payload)
    {:noreply, state}
  end

  defp setup_queue(id, chan) do
    {:ok, _} = Queue.declare(chan, @receive_queue, durable: true)
    :ok = Queue.bind(chan, @receive_queue, @receive_exchange)
  end
end
