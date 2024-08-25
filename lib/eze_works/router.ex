defmodule EzeWorks.Router do
  use Plug.Router

  plug(Plug.RequestId)
  plug(Plug.Logger)
  plug(Plug.Static, at: "assets", from: {:eze_works, "priv/assets"})
  plug(:match)
  plug(:dispatch)

  get "/" do
    render_page(conn, EzeWorks.Page.Home)
  end

  get "/post/:slug" do
    render_page(conn, EzeWorks.Page.Post, %{slug: slug})
  end

  get "/label/*label" do
    render_page(conn, EzeWorks.Page.Label, %{label: label})
  end

  match _ do
    send_resp(conn, 404, "oops")
  end

  def render_page(conn, module, opts \\ %{}) do
    html = module.page(opts) |> Pile.to_html()
    conn = put_resp_content_type(conn, "text/html")
    send_resp(conn, 200, html)
  end
end
