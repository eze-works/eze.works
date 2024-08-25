defmodule EzeWorks.Page.Post do
  def page(%{slug: slug}) do
    case EzeWorks.Store.fetch_by_slug(slug) do
      {:ok, post} ->
        EzeWorks.Page.base([
          {:h1, post.title},
          {:_rawtext, post.content}
        ])

      :error ->
        EzeWorks.Page.base({:h1, "No such post exists"})
    end
  end
end
