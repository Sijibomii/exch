defmodule Onion.UserSession do
  use GenServer, restart: :temporary

  defmodule Wallet do
    @type t :: %{
            id: String.t(),
            balance: number()
          }

    defstruct id: nil,
              balance: nil
  end


  defmodule State do
    @type t :: %__MODULE__{
            user_id: String.t(),
            email: String.t(),
            wallets: [Wallet.t()],
            ip: String.t(),
            pid: pid()
          }

    defstruct user_id: nil,
              ip: nil,
              pid: nil,
              email: nil,
              wallets: []
  end

   #################################################################################
  # REGISTRY AND SUPERVISION BOILERPLATE

  defp via(user_id), do: {:via, Registry, {Onion.UserSessionRegistry, user_id}}

  defp cast(user_id, params), do: GenServer.cast(via(user_id), params)
  defp call(user_id, params), do: GenServer.call(via(user_id), params)

  def start_supervised(initial_values) do
    callers = [self() | Process.get(:"$callers", [])]

    case DynamicSupervisor.start_child(
           Onion.UserSessionDynamicSupervisor,
           {__MODULE__, Keyword.merge(initial_values, callers: callers)}
         ) do
      {:error, {:already_started, pid}} -> {:ignored, pid}
      error -> error
    end
  end

  def child_spec(init), do: %{super(init) | id: Keyword.get(init, :user_id)}

  def count, do: Registry.count(Onion.UserSessionRegistry)

  def lookup(user_id), do: Registry.lookup(Onion.UserSessionRegistry, user_id)

  ###############################################################################
  ## INITIALIZATION BOILERPLATE

  def start_link(init) do
    GenServer.start_link(__MODULE__, init, name: via(init[:user_id]))
  end

  def init(init) do
    # transfer callers into the running process.
    Process.put(:"$callers", Keyword.get(init, :callers))
    {:ok, struct(State, init)}
  end

  ##############################################################################
  ## API HOOKS

  def set(user_id, key, value), do: cast(user_id, {:set, key, value})

  defp set_impl(key, value, state) do
    {:noreply, Map.put(state, key, value)}
  end

  def send_ws(user_id, platform, msg), do: cast(user_id, {:send_ws, platform, msg})

  defp send_ws_impl(_platform, msg, state = %{pid: pid}) do
    if pid, do: Ugwu.SocketHandler.remote_send(pid, msg)
    {:noreply, state}
  end

  def set_state(user_id, info), do: cast(user_id, {:set_state, info})

  defp set_state_impl(info, state) do
    {:noreply, Map.merge(state, info)}
  end

  def get(user_id, key), do: call(user_id, {:get, key})

  defp get_impl(key, _reply, state) do
    {:reply, Map.get(state, key), state}
  end

  def set_active_ws(user_id, pid), do: call(user_id, {:set_active_ws, pid})

  defp set_active_ws(pid, _reply, state) do
    if state.pid do
      # terminates another websocket that happened to have been running.
      Process.exit(state.pid, :normal)
    end

    Process.monitor(pid)
    {:reply, :ok, %{state | pid: pid}}
  end


  ##############################################################################
  ## MESSAGING API.
  ## TODO: change the first one to a call

  defp handle_disconnect(pid, state = %{pid: pid}) do
    {:stop, :normal, state}
  end

  defp handle_disconnect(_, state), do: {:noreply, state}

  #############################################################################
  ## ROUTER

  def handle_cast({:set, key, value}, state), do: set_impl(key, value, state)

  def handle_cast({:send_ws, platform, msg}, state),
    do: send_ws_impl(platform, msg, state)

  def handle_cast({:set_state, info}, state), do: set_state_impl(info, state)

  def handle_call(:get_info_for_msg, reply, state), do: get_info_for_msg_impl(reply, state)
  def handle_call({:get, key}, reply, state), do: get_impl(key, reply, state)
  def handle_call({:set_active_ws, pid}, reply, state), do: set_active_ws(pid, reply, state)

  def handle_info({:DOWN, _ref, :process, pid, _reason}, state), do: handle_disconnect(pid, state)

end
