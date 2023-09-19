defmodule Onion.OrderRabbit do
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
      Onion.OrderRabbitClientDynamicSupervisor,
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

  defp via(id), do: {:via, Registry, {Onion.OrderRabbitClientRegistry, id}}

  @send_queue "order"
  @receive_exchange "exch"

  def init(id) do
    IO.puts("order rabbits comming up")
    {:ok, conn} =
      Connection.open(Application.get_env(:egusi, :rabbit_url, "amqp://guest:guest@rabbits:5672/exch"))

    {:ok, chan} = Channel.open(conn)
    setup_queue(id, chan)

    {:ok, %State{chan: chan, id: id}}
  end

  def send(id, msg) do
    GenServer.cast(via(id), {:send, msg})
  end

  def handle_cast({:send, msg}, %State{chan: chan, id: id} = state) do
    AMQP.Basic.publish(chan, "exch", @send_queue, Jason.encode!(msg))
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
      _ -> :ok
    end

    # You might want to run payload consumption in separate Tasks in production
    # consume(chan, tag, redelivered, payload)
    {:noreply, state}
  end

  defp setup_queue(id, chan) do
    :ok
  end
end
