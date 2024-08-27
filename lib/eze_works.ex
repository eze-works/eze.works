defmodule EzeWorks do
  use Application

  def start(_type, _args) do
    children = [
      {EzeWorks.Store, get_posts()},
      {Bandit, plug: EzeWorks.Router, port: 3000}
    ]

    Supervisor.start_link(children, strategy: :one_for_one, name: EzeWorks)
  end

  defp get_posts() do
    paths = Path.wildcard("#{:code.priv_dir(:eze_works)}/posts/*.md")
    paths |> Enum.map(&load_post/1)
  end

  @metadata_fields [:title, :date, :labels]
  defp load_post(path) do
    {:ok, s} = :file.read_file(path, [:raw])
    [_, meta, post_content] = String.split(s, "+++", parts: 3)
    {meta, _} = Code.eval_string(meta)
    meta = Keyword.validate!(meta, @metadata_fields)
    html = MDEx.to_html(post_content, extension: [footnotes: true])

    %EzeWorks.Post{
      slug: Path.basename(path, ".md"),
      title: meta[:title],
      labels: meta[:labels],
      date: meta[:date],
      content: html
    }
  end
end
