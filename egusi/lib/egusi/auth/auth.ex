defmodule Egusi.Auth do

  alias Egusi.Utils.TokenUtils

  def authenticate(request, ip) do
    case TokenUtils.tokens_to_user_id(request.accessToken, request.refreshToken) do
      nil ->
        {:error, "invalid_authentication"}

      {:existing_claim, user_id} ->
        do_auth(user_id, nil, request, ip)
    end
  end

  # check the login session for the details of the user with this id
  defp do_auth(user_id, tokens, request, ip) do
    alias Onion.UserSession
    alias Onion.LoginSession

    case LoginSession.call(0, {:get_user_info, user_id}) do
      {:reply, user} -> UserSession.start_supervised(
          user_id: user.id,
          ip: ip,
          email: user.email,
          wallet: user.wallet,
          ip: ip,
          trading_client_id: user.trading_client_id
        )
        UserSession.set_active_ws(user.id, self())

        {:ok, user}
      _ -> {:close, 4001, "invalid_authentication"}
    end
  end

end
