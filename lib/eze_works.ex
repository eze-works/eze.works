defmodule EzeWorks do
  use Application

  def start(_type, _args) do
    :telemetry.attach(
      "web-server-telemetry",
      [:bandit, :request, :stop],
      &EzeWorks.Telemetry.dispatch_event/4,
      nil
    )

    children = [
      {EzeWorks.CodeReload, dirs: [Path.absname("lib"), Path.absname("priv")]},
      {EzeWorks.Store, get_posts()},
      {Bandit, plug: EzeWorks.Router, port: 3000},
      {Task.Supervisor, name: EzeWorks.Workers}
    ]

    Supervisor.start_link(children, strategy: :one_for_one, name: EzeWorks)
  end

  defp get_posts() do
    paths = Path.wildcard("#{:code.priv_dir(:eze_works)}/posts/*.md")
    paths |> Enum.map(&load_post/1)
  end

  @metadata_fields [:title, :date, :labels, :stage]
  defp load_post(path) do
    {:ok, s} = :file.read_file(path, [:raw])
    [meta, post_content] = String.split(s, "+++", parts: 2)
    {meta, _} = Code.eval_string(meta)
    meta = Keyword.validate!(meta, @metadata_fields)
    html = MDEx.to_html(post_content, extension: [footnotes: true])

    %EzeWorks.Post{
      slug: Path.basename(path, ".md"),
      title: meta[:title],
      labels: meta[:labels],
      date: meta[:date],
      stage: meta[:stage] || :published,
      content: html
    }
  end
end
