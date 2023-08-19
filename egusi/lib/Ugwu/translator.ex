defmodule Ugwu.Translator do
  import Egusi.Utils.Version
  alias Ugwu.Translator.V0_1_0
  require V0_1_0

  def translate_inbound(message) when V0_1_0.translates(message) do
    V0_1_0.translate_inbound(message)
  end

  def translate_outbound(message, original = %{version: ~v(0.1.0)}) do
    V0_1_0.translate_outbound(message, original)
  end

  def translate_outbound(message, _), do: message
end
