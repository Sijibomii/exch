defmodule Onion.TickerSession do
  use GenServer, restart: :temporary


  defmodule Order do
    @type t :: %{
            id: String.t(),
            side: String.t(),
            operation: String.t(),
            time: String.t(),
            volume: number(),
            seq_num: number(),
            price: number()
          }

    defstruct id: nil,
              side: nil,
              operation: nil,
              time: nil,
              volume: nil,
              seq_num: nil,
              price: nil
  end

  defmodule Node do
    @type t :: %{
      order: Order.t(),
      prev: Order.t(),
      next: Order.t()
    }

    defstruct order: nil,
              prev: nil,
              next: nil
  end

  defmodule Queue do
    @type t :: %{
      head: Node.t(),
      tail: Node.t()
    }
    defstruct head: nil,
              tail: nil
  end

  defmodule Data do

    @derive {Jason.Encoder, only: ~w(
      time
      open
      close
      high
      low
    )a}


    @type t :: %{
      time: integer(),
      open: String.t(),
      close: String.t(),
      high: String.t(),
      low: String.t()
    }

    defstruct time: nil,
              open: "",
              close: "",
              high: "",
              low: ""
  end

  defmodule State do
    @type t :: %__MODULE__{
            ticker_id: String.t(),
            order_book: Queue.t(),
            listeners: [any()],
            last_update_time: integer(),
            chart_data: [Data.t()],
            current_open: String.t(),
            current_high: String.t(),
            current_low: String.t()
          }

    defstruct ticker_id: "",
              order_book: %Queue{ head: nil, tail: nil },
              listeners: [],
              last_update_time: nil,
              current_open: nil,
              current_high: nil,
              current_low: nil,
              chart_data: []
  end

  #TODO: send cast message to start a task after ticker session is started


  #################################################################################
  # REGISTRY AND SUPERVISION BOILERPLATE

  defp via(ticker_id), do: {:via, Registry, {Onion.TickerSessionRegistry, ticker_id}}

  defp cast(ticker_id, params), do: GenServer.cast(via(ticker_id), params)
  defp call(ticker_id, params), do: GenServer.call(via(ticker_id), params)

  def start_supervised(initial_values) do
    callers = [self() | Process.get(:"$callers", [])]

    case DynamicSupervisor.start_child(
           Onion.TickerSessionDynamicSupervisor,
           {__MODULE__, Keyword.merge(initial_values, callers: callers)}
         ) do
      {:error, {:already_started, pid}} -> {:ignored, pid}
      error -> error
    end
  end

  def child_spec(init), do: %{super(init) | id: Keyword.get(init, :ticker_id)}

  def count, do: Registry.count(Onion.TickerSessionRegistry)
  def lookup(user_id), do: Registry.lookup(Onion.TickerSessionRegistry, user_id)


  ###############################################################################
  ## INITIALIZATION BOILERPLATE
  def start_link(init) do
    GenServer.start_link(__MODULE__, init, name: via(init[:ticker_id]))
  end

  def init(init) do
    # adopt callers from the call point.
    Process.put(:"$callers", init[:callers])
    {:ok, struct(State, init)}
  end

  ########################################################################
  ## API

  def add_order(ticker_id, msg), do: cast(ticker_id, {:incremental, msg})

  def request_orderbook(ticker_id), do: call(ticker_id, :request_orders)

  def join_ticker_updates(user_trading_id, ticker_id), do: cast(ticker_id, {:listen, user_trading_id})
  ########################################################################
  ## impl

  defp get_orderbook_impl(_reply, state) do
    # let the caller take care of encoding
    {:reply, {:ok, Enum.reverse(state.chart_data)}, state}
  end

  def add_listener(ticker_id, user_trading_id), do: cast(ticker_id, {:listen, user_trading_id})

  defp add_listener_impl(user_trading_id, state) do
    {:noreply, %{state | listeners: [user_trading_id | state.listeners]}}
  end


  #########################################################################
  ##list impl
  defp empty?(%Queue{head: nil, tail: nil}), do: true

  defp empty?(_), do: false

  defp append(list, value) when is_nil(list.head) do
    node = %Node{order: value, prev: nil, next: nil}
    %Queue{list | head: node, tail: node}
  end

  defp append(list, value) do
    new_node = %Node{order: value, prev: list.tail, next: nil}
    old_tail = list.tail
    updated_old_tail = %{old_tail | next: new_node}
    %Queue{list | tail: new_node}
  end

  defp prepend(list, value) when is_nil(list.head) do
    node = %Node{order: value, prev: nil, next: nil}
    %Queue{list | head: node, tail: node}
  end

  defp prepend(list, value) do
    new_node = %Node{order: value, prev: nil, next: list.head}
    old_head = list.head
    updated_old_head = %{old_head | prev: new_node}
    %Queue{list | head: new_node}
  end

  defp pop_head(list) when is_nil(list.head) do
    list
  end

  defp pop_head(list) do
    new_head = list.head.next
    prev_head = list.head
    case new_head do
      nil -> %Queue{tail: nil }
      _ ->
        updated_new_head = %{new_head | prev: nil}
        {prev_head, %Queue{list | head: updated_new_head}}
    end
  end

  defp pop_tail(list) when is_nil(list.tail) do
    list
  end

  defp pop_tail(list) do
    new_tail = list.tail.prev
    case new_tail do
      nil -> %Queue{head: nil}
      _ ->
        updated_new_tail = %{new_tail | next: nil}
        %Queue{list | tail: updated_new_tail}
    end
  end

  defp to_list(queue) when is_nil(queue.head) do
    []
  end

  defp to_list(queue) do
    {value, rest } = pop_head(queue)
    [value | to_list(rest)]
  end


  ########################################################################
  ## ROUTER

  def handle_cast({:incremental, message}, state) do
    # if op is trading send ws message to all listeners
    if message["op"] == "MARKET-UPDATE-TRADE" do
      IO.puts("incremental called for trade!!")
      IO.inspect(state.listeners)
      current_timestamp = System.system_time(:millisecond)
      if is_nil(state.last_update_time) or is_nil(state.current_high) or is_nil(state.current_low) do
        # just started trading
        Enum.each(state.listeners, fn tid ->
          Onion.UserSession.send_ws(tid, %{
            ref: UUID.uuid4(),
            op: "MARKET-UPDATE-NEW-TRADE",
            data: %{
              time: current_timestamp,
              open: 0,
              close: message["data"]["price"],
              high: message["data"]["price"],
              low: 0
            }
          })
        end)

        {:noreply, %{state | order_book: append(state.order_book, %Order{
          id: message["data"]["id"],
          side: message["data"]["side"],
          operation: message["op"],
          time: System.system_time(:millisecond),
          volume: message["data"]["qty"],
          seq_num: message["data"]["seq_num"],
          price: message["data"]["price"],
        }),
          chart_data: [%Data{
          time: current_timestamp,
          open: 0,
          close: message["data"]["price"],
          high: message["data"]["price"],
          low: 0
          } | state.chart_data],
          last_update_time: current_timestamp,
          current_open: 0,
          current_high: message["data"]["price"],
          current_low: 0
          }}
      else
        elapsed_time_ms = current_timestamp - state.last_update_time
        if elapsed_time_ms >= 20000 do
          # more that 20 sec so new candle stick. how does it know when to start building a new candlestick

          case state.chart_data do
            [head | _tail] ->
              ht = if head.close > message["data"]["price"], do: head.close, else: message["data"]["price"]
              lw = if head.close < message["data"]["price"], do: head.close, else: message["data"]["price"]
              Enum.each(state.listeners, fn tid ->
                Onion.UserSession.send_ws(tid, %{
                  ref: UUID.uuid4(),
                  op: "MARKET-UPDATE-NEW-TRADE",
                  data: %{
                    time: current_timestamp,
                    open: head.close,
                    close: message["data"]["price"],
                    high: ht,
                    low: lw
                  }
                })
              end)

              {:noreply, %{state | order_book: append(state.order_book, %Order{
                id: message["data"]["id"],
                side: message["data"]["side"],
                operation: message["op"],
                time: System.system_time(:millisecond),
                volume: message["data"]["qty"],
                seq_num: message["data"]["seq_num"],
                price: message["data"]["price"],
              }),
                chart_data: [%Data{
                time: current_timestamp,
                open: head.close,
                close: message["data"]["price"],
                high: ht,
                low: lw
                } | state.chart_data],

                last_update_time: current_timestamp,
                current_open: head.close,
                current_high: ht,
                current_low: lw
                }}

            _ ->
              # this should never happen
              IO.puts("empty ooo")
              Enum.each(state.listeners, fn tid ->
              Onion.UserSession.send_ws(tid, %{
                ref: UUID.uuid4(),
                op: "MARKET-UPDATE-NEW-TRADE",
                data: %{
                  time: current_timestamp,
                  open: message["data"]["price"],
                  close: message["data"]["price"],
                  high: message["data"]["price"],
                  low: message["data"]["price"]
                }
              })
              end)

              {:noreply, %{state | order_book: append(state.order_book, %Order{
                id: message["data"]["id"],
                side: message["data"]["side"],
                operation: message["op"],
                time: System.system_time(:millisecond),
                volume: message["data"]["qty"],
                seq_num: message["data"]["seq_num"],
                price: message["data"]["price"],
              }),
                chart_data: [%Data{
                time: current_timestamp,
                open: message["data"]["price"],
                close: message["data"]["price"],
                high: message["data"]["price"],
                low: message["data"]["price"]
                } | state.chart_data],
                last_update_time: current_timestamp,
                current_open: message["data"]["price"],
                current_high: message["data"]["price"],
                current_low: message["data"]["price"]
                }}

          end
        else
          IO.inspect(state)
          current_high = cond do
            state.current_high > message["price"] -> state.current_high
            true -> message["price"]
          end
          current_low = cond do
            state.current_low < message["price"] -> state.current_low
            true -> message["price"]
          end
          # current_low: message["price"]
          Enum.each(state.listeners, fn tid ->
            Onion.UserSession.send_ws(tid, %{
              ref: UUID.uuid4(),
              op: "MARKET-UPDATE-NEW-TRADE",
              data: %{
                time: state.last_update_time,
                open: state.current_open,
                close: message["price"],
                high: current_high,
                low: current_low
              }
            })
          end)

          {:noreply, %{state | order_book: append(state.order_book, %Order{
            id: message["id"],
            side: message["side"],
            operation: message["op"],
            time: System.system_time(:millisecond),
            volume: message["qty"],
            seq_num: message["seq_num"],
            price: message["price"],
          }),
            chart_data: [%Data{
            time: state.last_update_time,
            open: state.current_open,
            close: message["price"],
            high: current_high,
            low: current_low
            } | state.chart_data],
            last_update_time: state.last_update_time,
            current_high: current_high,
            current_low: current_low
            }}

        end
      end
    else
      {:noreply, state}
    end
  end

  def handle_cast({:listen, user_trading_id}, state), do: add_listener_impl(user_trading_id, state)

  def handle_call(:request_orders, reply, state), do: get_orderbook_impl(reply, state)


end
