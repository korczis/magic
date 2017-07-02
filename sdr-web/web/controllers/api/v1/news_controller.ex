defmodule SdrWeb.API.V1.NewsController do
  use SdrWeb.Web, :controller

  require Logger

  def test(conn, _params) do
    conn
    |> json(%{message: "This is test!"})
  end
end
