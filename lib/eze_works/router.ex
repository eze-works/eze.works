defmodule EzeWorks.Router do
  use Plug.Router

  plug(Plug.RequestId)
  plug(Plug.Logger)
  plug(Plug.Static, at: "assets", from: {:eze_works, "priv/assets"})
  plug(:match)
  plug(:dispatch)

  get "/" do
    posts = EzeWorks.Store.get_posts()
    render_page(conn, :home, %{posts: posts})
  end

  get "/post/:slug" do
    case EzeWorks.Store.get_post(slug) do
      {:ok, post} -> render_page(conn, :single_post, %{post: post})
      :notfound -> render_404(conn)
    end
  end

  match _ do
    render_404(conn)
  end

  defp render_page(conn, page, opts) do
    html = apply(EzeWorks.PageHandler, page, [opts]) |> Pile.to_html(doctype: true, iodata: true)
    conn = put_resp_content_type(conn, "text/html")
    send_resp(conn, 200, html)
  end

  def render_404(conn) do
    html = EzeWorks.PageHandler.not_found() |> Pile.to_html(doctype: true, iodata: true)
    conn = put_resp_content_type(conn, "text/html")
    send_resp(conn, 404, html)
  end
end
