defmodule Egusi.Trade.Cancel do

  alias Onion.UserSession


  def cancel(
    user_trading_id,
    ticker_id,
    order_id
    ) do

    case UserSession.cancel_trade(user_trading_id, {:cancel_trade, %{
      order_id: order_id,
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
