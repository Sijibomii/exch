defmodule Egusi.Utils.TokenUtils do

  alias Egusi.Utils.UUID

  def tokens_to_user_id(access_token!, _refresh_token) do
    access_token! = access_token! || ""

    case Kousa.AccessToken.verify_and_validate(access_token!) do
      {:ok, claims} ->
        {:existing_claim, claims["user"]["id"]}

      _ -> nil
    end
  end

end
