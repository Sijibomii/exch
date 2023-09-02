defmodule Onion.TickerSession do
  use GenServer, restart: :temporary


  defmodule Order do
    @type t :: %{
            id: String.t(),
            user_id: String.t(),
            side: String.t(),
            operation: String.t(),
            time: String.t(),
            volume: number()
          }

    defstruct id: nil,
              user_id: nil,
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


end
