defmodule EzeWorks.Page.Post do
  def page(%{slug: slug}) do
    case EzeWorks.Store.fetch_by_slug(slug) do
      {:ok, post} ->
        EzeWorks.Page.base(
          {:div, %{class: "post__content"}, {:h1, post.title}, {:_rawtext, post.content}},
          title: post.title
        )

      :error ->
        EzeWorks.Page.base({:h1, "No such post exists"})
    end
  end
end
