defmodule Egusi.AccessToken do
  def __default_signer__,
    do: Joken.Signer.create("RS256", Application.fetch_env!(:egusi, ":access_token_secret"))

  use Joken.Config

  # 1 hour
  def token_config, do: default_claims(default_exp: 60 * 60)
end
