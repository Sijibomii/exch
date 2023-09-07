defmodule Ugwu.Translator do

  ############################################################################
  ## INBOUND MESSAGES

  @operator_translations %{
    "add_new_trade" => "trade:new",
  }

  @operators Map.keys(@operator_translations)

  defguard translates(message) when :erlang.map_get("op", message) in @operators

  def translate_inbound(message = %{"op" => operator}) do
    message
    |> translate_operation
    |> translate_in_body(operator)
    |> add_in_ref(operator)
  end

  def translate_operation(message = %{"op" => operator}) do
    put_in(message, ["op"], @operator_translations[operator])
  end

  def translate_in_body(message, _op), do: message
  def add_in_ref(message, _op), do: message


  ############################################################################
  ## OUTBOUND MESSAGES

  # out boubd translations here

  defp add_out_ref(message, %{ref: ref}), do: Map.put(message, :fetchId, ref)
  defp add_out_ref(message, _), do: message

  defp add_out_err(message, %{e: err}), do: Map.put(message, :e, err)
  defp add_out_err(message, _), do: message

  def translate_out_body(message, "auth:request") do
    %{message | op: "auth-good", d: %{user: message.d}}
  end

  #################################################################
  # pure outbound messages

  def translate_out_body(message, _), do: message
end
