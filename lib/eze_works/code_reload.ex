defmodule EzeWorks.CodeReload do
  use GenServer

  def init(args) do
    {:ok, watcher_pid} = FileSystem.start_link(args)
    FileSystem.subscribe(watcher_pid)
    {:ok, %{watcher_pid: watcher_pid}}
  end

  def start_link(args) do
    GenServer.start_link(__MODULE__, args)
  end

  def handle_info({:file_event, watcher_pid, _}, %{watcher_pid: watcher_pid} = state) do
    Mix.Task.reenable("compile.elixir")
    Mix.Task.run("compile.elixir")
    {:noreply, state}
  end

  def handle_info({:file_event, watcher_pid, :stop}, %{watcher_pid: watcher_pid} = state) do
    {:noreply, state}
  end
end
