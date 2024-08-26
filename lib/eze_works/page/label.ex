defmodule EzeWorks.Page.Label do
  def page(%{label: label}) do
    hierarchy = EzeWorks.Store.fetch_label_hierarchy(label)

    html =
      Enum.map(hierarchy, fn {label, posts} ->
        {:ul, {:span, %{class: "label"}, label |> Enum.join("/")},
         Enum.map(posts, fn post ->
           {:li, {:a, %{href: "/post/#{post.slug}"}, post.title}}
         end)}
      end)

    EzeWorks.Base.base(html)
  end
end
