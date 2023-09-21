defmodule Ugwu do
  import Plug.Conn

  use Plug.Router
  use Plug.Builder

  plug Corsica, origins: "*"

  if Mix.env() == :test do
    plug(:set_callers)

    defp get_callers(%Plug.Conn{req_headers: req_headers}) do
      {_, request_bin} = Enum.find(req_headers, fn {key, _} -> key == "user-agent" end)

      List.wrap(
        if is_binary(request_bin) do
          request_bin
          |> Base.decode16!()
          |> :erlang.binary_to_term()
        end
      )
    end

    defp set_callers(conn, _params) do
      Process.put(:"$callers", get_callers(conn))
      conn
    end
  end


  plug(Ugwu.Plugs.Cors)
  plug(:match)
  plug(:dispatch)

  options _ do
    send_resp(conn, 200, "")
  end

  get "/" do
    conn
    |> put_resp_content_type("application/json")
    |> send_resp(
      200,
      Jason.encode!(%{hello: "hello world"})
    )
  end


  get _ do
    send_resp(conn, 404, "not found")
  end

  post _ do
    send_resp(conn, 404, "not found")
  end
end
