defmodule EzeWorks.PageHandler do
  def home(%{posts: posts}) do
    base_layout([
      {:div, %{class: "featured-quote"}, featured_quote()},
      {:div, %{class: "post-list center"}, Enum.map(posts, &post_card/1)}
    ])
  end

  defp post_card(post) do
    {
      :div,
      %{class: "post-card"},
      {:a, %{class: "post-card-title", href: "/post/#{post.slug}"}, post.title},
      {:span, %{class: "post-card-labels"}, Enum.map(post.labels, &label/1)},
      {:span, %{class: "post-card-date"}, Calendar.strftime(post.date, "%b %d, %Y")}
    }
  end

  defp label(label) do
    {:span, %{class: "post-label"}, label}
  end

  defp featured_quote() do
    [
      {:p,
       "We build our computers the way we build our cities – over time, without a plan, on top of ruins."},
      {:p, "– Ellen Ullman, Life in Code"}
    ]
  end

  def single_post(%{post: post}) do
    base_layout([
      {
        :div,
        %{class: "post-container center"},
        {
          :div,
          %{class: "post-meta"},
          {:h1, %{class: "post-title"}, post.title},
          {:span, %{class: "post-labels"}, Enum.map(post.labels, &label/1)},
          {:span, %{class: "post-date"}, Calendar.strftime(post.date, "%b %d, %Y")}
        },
        {:div, %{class: "post-content"}, {:_rawtext, post.content}}
      }
    ])
  end

  def not_found() do
    base_layout(
      [
        {
          :div,
          %{class: "center not-found"},
          {:h1, "The page you requested could not be found"}
        }
      ],
      title: "Page Not Found"
    )
  end

  @base_options [title: "Home"]
  def base_layout(content, opts \\ @base_options) do
    opts = Keyword.validate!(opts, @base_options)

    [
      {
        :html,
        head(opts),
        {
          :body,
          {
            :div,
            %{id: "main-content"},
            {:div, %{id: "logo", class: "center"}, {:a, %{href: "/"}, "e.w"}},
            content
          },
          footer()
        }
      }
    ]
  end

  defp head(opts) do
    {
      :head,
      metadata(opts),
      css(),
      js()
    }
  end

  defp metadata(opts) do
    viewport = "width=device-width, initial-scale=1, shrink-to-fit=no"

    [
      {:meta, %{charset: "utf-8"}},
      {:meta, %{name: "viewport", content: viewport}},
      {:title, "Eze Works | #{opts[:title]}"}
    ]
  end

  defp css() do
    [
      {:link, %{rel: "stylesheet", href: "/assets/css/reset.css"}},
      {:link, %{rel: "stylesheet", href: "/assets/css/fonts.css"}},
      {:link, %{rel: "stylesheet", href: "/assets/css/styles.css"}}
    ]
  end

  defp js() do
    [
      {:script, %{src: "/assets/js/htmx.min.js"}}
    ]
  end

  @license_link "https://creativecommons.org/licenses/by-sa/4.0/"
  defp footer() do
    {
      :footer,
      {:p, "This site's content is licensed under ", {:a, %{href: @license_link}, "CC-BY-SA"}}
    }
  end
end
