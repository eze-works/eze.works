defmodule EzeWorks.Store do
  @moduledoc """
  Post storage

  Posts are loaded up into an `Agent` process when the application stores.
  The in-memory state of the agent is defined by this [struct](t:EzeWorks.Store.t/0)
  """

  @typedoc """
  A post index.

  The actual posts as values under the `by_slug` attributes.
  All other attributes store slugs. 
  """
  @type t :: %__MODULE__{
          by_slug: %{String.t() => %EzeWorks.Post{}},
          by_label: %{[String.t()] => [String.t()]}
        }

  defstruct by_slug: %{}, by_label: %{}

  use Agent

  def start_link(posts) do
    Agent.start_link(fn -> init(posts) end, name: __MODULE__)
  end

  defp init(posts) do
    store = %__MODULE__{
      by_slug: %{},
      by_label: %{}
    }

    store = Enum.reduce(posts, store, &add_post_to_store/2)
    store
  end

  defp add_post_to_store(%EzeWorks.Post{} = post, %EzeWorks.Store{} = store) do
    # Store the post in a map, keyed by the slug
    # One post per slug
    slug_store = Map.put_new(store.by_slug, post.slug, post)

    # Store slugs in a map keyed by labels
    # Multiple posts per label
    post_labels = Enum.map(post.labels, fn l -> String.split(l, "/", trim: true) end)

    label_store =
      Enum.reduce(post_labels, store.by_label, fn label, label_store ->
        Map.update(label_store, label, [post.slug], fn existing -> [post.slug | existing] end)
      end)

    # Update the store
    %{store | by_slug: slug_store, by_label: label_store}
  end

  def list_posts() do
    Agent.get(__MODULE__, fn store ->
      store.by_slug
      |> Map.values()
      |> Enum.sort_by(fn post -> post.date end, {:desc, Date})
    end)
  end

  def fetch_by_slug(slug) do
    filtered = Agent.get(__MODULE__, fn store -> store_filter_by_slugs(store, [slug]) end)
    Map.fetch(filtered, slug)
  end

  def fetch_label_hierarchy(label) do
    Agent.get(__MODULE__, fn store -> store_filter_by_label(store, label) end)
  end

  defp store_filter_by_slugs(store, slugs) do
    store.by_slug
    |> Map.filter(fn {slug, _} -> Enum.member?(slugs, slug) end)
  end

  defp store_filter_by_label(store, label) do
    store.by_label
    |> Map.filter(fn {l, _} -> List.starts_with?(l, label) end)
    |> Enum.map(fn {l, slugs} -> {l, store_filter_by_slugs(store, slugs) |> Map.values()} end)
  end
end
