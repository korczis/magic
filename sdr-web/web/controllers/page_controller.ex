defmodule SdrWeb.PageController do
  use SdrWeb.Web, :controller

  def index(conn, _params) do
    render conn, "index.html"
  end
end
