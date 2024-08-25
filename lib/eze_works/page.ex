defmodule EzeWorks.Page do
  @base_options [title: "Home"]

  @logo File.read!("priv/assets/svg/logo.svg")

  def base(content, opts \\ @base_options) do
    opts = Keyword.validate!(opts, @base_options)
    {:html,
     [
       {:head,
        [
          {:meta, %{charset: "utf-8"}},
          {:meta,
           %{
             name: "viewport",
             content: "width=device-width, initial-scale=1, shrink-to-fit=no"
           }},
          {:title, "Eze Works | #{opts[:title]}"},
          {:link, %{rel: "stylesheet", href: "/assets/css/reset.css"}},
          {:link, %{rel: "stylesheet", href: "/assets/css/base.css"}}
        ]},
       {:body,
        [
          {:header, %{class: "max-width"}, header()},
          {:main, %{class: "max-width"}, content}
        ]}
     ]}
  end

  def header() do
    [
      {:span, %{id: "logo"}, {:_rawtext, @logo}},
      {:nav,
       {:ul,
        [
          {:li, {:a, %{href: "/"}, "Home"}}
        ]}}
    ]
  end
end
