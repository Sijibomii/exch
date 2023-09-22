defmodule Ugwu.Translator do

  alias Ugwu.Translator.V0_1_0
  require V0_1_0

  def translate_inbound(message) when V0_1_0.translates(message) do
    V0_1_0.translate_inbound(message)
  end
  # only auth msg request should come here
  def translate_outbound(message, original = %{ inbound_operator: "auth:request" }) do
    V0_1_0.translate_outbound(message, original)
  end

  def translate_outbound(message, _), do: message
end
