defmodule Egusi.Utils.TokenUtils do

  alias Egusi.Utils.UUID

  def tokens_to_user_id(access_token!, _refresh_token) do
    access_token! = access_token! || ""

    # private_key_path = "path/to/private_key.pem"
    public_key_path = "/keys/public_key.pem"
    # {:ok, private_key} = File.read(private_key_path)
    {:ok, public_key} = File.read(public_key_path)
    {_, %{"n" => n} = key_map} =
      JOSE.JWK.from_pem(public_key)
      |> JOSE.JWK.to_map()

    signer = Joken.Signer.create("RS256", key_map)

    case Egusi.AccessToken.verify_and_validate(access_token!, signer) do
      {:ok, claims} ->
        # IO.puts("validating claims: #{claims} ")
        {:existing_claim, claims["user"]["id"]}

      _ ->
        IO.puts("validating claims: ERROR VALIDATING CLAIMS ")
        nil
    end
  end

end
