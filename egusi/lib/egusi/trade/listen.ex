defmodule Egusi.Trade.Listen do

  alias Onion.UserSession

  def listen(user_trading_id, ticker_id) do

    case UserSession.listen_trades(user_trading_id, ticker_id) do

      :ok -> {:ok, %{trade: %{ ticker_id: ticker_id }}}

      {:error, reason} -> {:error, reason}

      _ -> {:error, "failed to listen to trade!"}
    end

  end
end
