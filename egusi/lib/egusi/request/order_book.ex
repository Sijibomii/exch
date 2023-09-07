defmodule Egusi.Request do

  alias Onion.TickerSession

  def request_orderbook(
    user_trading_id,
    ticker_id) do

    case TickerSession.request_orderbook(ticker_id) do
      {:ok, orders} -> {:ok, %{ orders: orders }}

      {:error, reason} -> {:error, reason}

      _ -> {:error, "error getting orderbook"}
    end

  end
end
