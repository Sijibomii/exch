defmodule Onion.TickerSession do
  use GenServer, restart: :temporary


  defmodule Order do
    @type t :: %{
            id: String.t(),
            side: String.t(),
            operation: String.t(),
            time: String.t(),
            volume: number()
          }

    defstruct id: nil,
              side: nil,
              operation: nil,
              time: nil,
              volume: nil
  end

  defmodule State do
    @type t :: %__MODULE__{
            ticker_id: String.t(),
            trading_id: String.t(),
            order_book: [Order.t()]
          }

    defstruct ticker_id: "",
              trading_id: "",
              order_book: []
  end


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


  ########################################################################
  ## ROUTER



end
