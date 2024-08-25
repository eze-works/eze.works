defmodule EzeWorks.Page do
  @base_options [title: "Home"]

  def base(content, opts \\ @base_options) do
    opts = Keyword.validate!(opts, @base_options)

    {
      :html,
      {
        :head,
        {:meta, %{charset: "utf-8"}},
        {:meta,
         %{
           name: "viewport",
           content: "width=device-width, initial-scale=1, shrink-to-fit=no"
         }},
        {
          :title,
          "Eze Works | #{opts[:title]}"
        },
        {:link, %{rel: "stylesheet", href: "/assets/css/reset.css"}}
      },
      {
        :body,
        {:div, %{id: "content"}, {:header, header()}, {:main, content},
         {
           :footer,
           {
             :p,
             "This site's content is licensed under ",
             {
               :a,
               %{href: "https://creativecommons.org/licenses/by-sa/4.0/"},
               "CC-BY-SA"
             }
           },
           {
             :p,
             "The cog icon was created by ",
             {
               :a,
               %{href: "https://thenounproject.com/creator/GreenHill/"},
               "GreenHill"
             },
             " from ",
             {
               :a,
               %{href: "https://thenounproject.com/"},
               "the noun project"
             }
           }
         }}
      }
    }
  end

  def header() do
    []
  end
end
