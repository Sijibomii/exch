defmodule Egusi.Trade.Create do

  alias Onion.UserSession
  alias Onion.TickerSession

  def create(
    user_trading_id,
    ticker_id,
    side,
    price,
    qty
  ) do

    case UserSession.new_trade(user_trading_id, {:new_trade, %{
      ticker_id: ticker_id,
      side: side,
      price: price,
      qty: qty
    }}) do

      {:ok} -> {:ok, %{trade: %{
        ticker_id: ticker_id,
        side: side,
        price: price,
        qty: qty
      }}}

      {:error, reason} -> {:error, reason}

      _ -> {:error, "trade could no be placed"}
    end

  end
end
