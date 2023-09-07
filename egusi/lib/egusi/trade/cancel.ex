defmodule Egusi.Trade do

  alias Onion.UserSession
  alias Onion.TickerSession

  def cancel(
    user_trading_id,
    ticker_id
    ) do

    case UserSession.cancel_trade(user_trading_id, {:cancel_trade, %{
      ticker_id: ticker_id,
    }}) do

      {:ok} -> {:ok, %{trade: %{
        ticker_id: ticker_id,
      }}}

      {:error, reason} -> {:error, reason}

      _ -> {:error, "trade could no be modified!"}
    end

  end
end
