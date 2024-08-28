defmodule EzeWorks.Store do
  @moduledoc """
  Post storage using ets tables
  """
  use GenServer

  def start_link(posts) do
    GenServer.start_link(__MODULE__, posts)
  end

  @impl true
  def init(posts) do
    # this table stores posts keyed by their slug. so it's 1:1
    :ets.new(:by_slug, [:named_table, {:read_concurrency, true}])
    # this table stores posts keyed by label, so it's 1:many
    :ets.new(:by_label, [:bag, :named_table, {:read_concurrency, true}])
    {:ok, posts, {:continue, :load_posts}}
  end

  @impl true
  def handle_continue(:load_posts, posts) do
    for post <- posts do
      :ets.insert(:by_slug, {post.slug, post})

      for label <- post.labels do
        :ets.insert(:by_label, {label, post})
      end
    end

    {:noreply, []}
  end

  @doc """
  Returns any non-draft posts in descending order of publication date
  """
  def get_posts() do
    :ets.tab2list(:by_slug)
    |> Enum.map(fn {_key, value} -> value end)
    |> Enum.reject(fn post -> post.stage == :draft end)
    |> Enum.sort_by(fn post -> post.date end, {:desc, Date})
  end

  def get_post(slug) do
    case :ets.lookup(:by_slug, slug) do
      [] -> :notfound
      [{_key, value}] -> {:ok, value}
    end
  end
end
