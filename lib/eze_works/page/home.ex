defmodule EzeWorks.Page.Home do
  def page(_args) do
    posts = EzeWorks.Store.list_posts()

    EzeWorks.Page.base(
      {:ul,
       Enum.map(posts, fn post ->
         {
           :li,
           {:a, %{href: "/post/#{post.slug}"}, post.title},
           Enum.map(post.labels, fn label ->
             {
               :a,
               %{href: "/label/#{label}", class: "label"},
               label
             }
           end)
         }
       end)}
    )
  end
end
